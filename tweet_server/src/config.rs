use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;

use actix_web::cookie::Key;

/// Runtime configuration, resolved from environment variables once at startup.
///
/// | Variable                | Default            | Purpose                                              |
/// |-------------------------|--------------------|------------------------------------------------------|
/// | `DATABASE_URL`          | required           | PostgreSQL connection string                         |
/// | `HOST`                  | `0.0.0.0`          | Bind interface                                       |
/// | `PORT`                  | `8000`             | Bind port; `0` picks a free port                     |
/// | `PORT_FILE`             | `.server-port`     | Where the actually bound port is written             |
/// | `UPLOAD_DIR`            | `uploads`          | Directory for uploaded files                         |
/// | `SESSION_SECRET_KEY`    | random             | Cookie signing key (>= 32 chars) so sessions persist |
/// | `RATE_LIMIT_PER_SECOND` | `20`               | Per-IP sustained request rate                        |
/// | `RATE_LIMIT_BURST`      | `50`               | Per-IP burst allowance                               |
/// | `TRUST_PROXY`           | `false`            | Rate-limit by `X-Forwarded-For` / `X-Real-IP`        |
/// | `CORS_ALLOWED_ORIGINS`  | unset (any origin) | Comma-separated origins allowed cross-origin         |
/// | `SESSION_COOKIE_SECURE` | `false`            | Send the session cookie over HTTPS only              |
/// | `DB_MAX_CONNECTIONS`    | `10`               | PostgreSQL pool size                                 |
/// | `DB_ACQUIRE_TIMEOUT_SECS` | `5`              | Max wait for a pooled connection before a 500        |
/// | `MAX_UPLOAD_MB`         | `300`              | Max total size of one upload request                 |
/// | `SHUTDOWN_TIMEOUT_SECS` | `30`               | Grace period for in-flight requests on shutdown      |
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub port_file: PathBuf,
    pub upload_dir: PathBuf,
    pub rate_per_second: u64,
    pub rate_burst: u32,
    pub trust_proxy: bool,
    /// `None` keeps the permissive development default (any origin).
    pub cors_origins: Option<Vec<String>>,
    pub cookie_secure: bool,
    pub db_max_connections: u32,
    pub db_acquire_timeout_secs: u64,
    pub max_upload_bytes: u64,
    pub shutdown_timeout_secs: u64,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let upload_dir = PathBuf::from(env_or("UPLOAD_DIR", "uploads"));
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            host: env_or("HOST", "0.0.0.0"),
            port: env_parse("PORT", 8000),
            port_file: PathBuf::from(env_or("PORT_FILE", ".server-port")),
            upload_dir: std::path::absolute(&upload_dir).unwrap_or(upload_dir),
            rate_per_second: env_parse("RATE_LIMIT_PER_SECOND", 20),
            rate_burst: env_parse("RATE_LIMIT_BURST", 50),
            trust_proxy: env_bool("TRUST_PROXY", false),
            cors_origins: std::env::var("CORS_ALLOWED_ORIGINS")
                .ok()
                .map(|v| parse_origins(&v)),
            cookie_secure: env_bool("SESSION_COOKIE_SECURE", false),
            db_max_connections: env_parse("DB_MAX_CONNECTIONS", 10u32).max(1),
            db_acquire_timeout_secs: env_parse("DB_ACQUIRE_TIMEOUT_SECS", 5),
            max_upload_bytes: env_parse("MAX_UPLOAD_MB", 300u64).saturating_mul(1024 * 1024),
            shutdown_timeout_secs: env_parse("SHUTDOWN_TIMEOUT_SECS", 30),
        }
    }

    /// Session cookie key. Derived from `SESSION_SECRET_KEY` when present so
    /// sessions survive restarts; otherwise random (and a warning is logged).
    pub fn session_key(&self) -> Key {
        match std::env::var("SESSION_SECRET_KEY") {
            Ok(secret) if secret.len() >= 32 => Key::derive_from(secret.as_bytes()),
            Ok(_) => {
                tracing::warn!("SESSION_SECRET_KEY is shorter than 32 bytes; using a random key");
                Key::generate()
            }
            Err(_) => {
                tracing::warn!("SESSION_SECRET_KEY not set; sessions will reset on restart");
                Key::generate()
            }
        }
    }

    /// Log the bound address and write the port to `PORT_FILE` so the
    /// frontend dev proxy can find the server even when `PORT=0`.
    pub fn publish_bound_addrs(&self, addrs: &[SocketAddr]) {
        let Some(addr) = addrs.first() else { return };
        tracing::info!("listening on http://{addr}");
        let written =
            std::fs::File::create(&self.port_file).and_then(|mut f| writeln!(f, "{}", addr.port()));
        if let Err(e) = written {
            tracing::warn!("could not write {}: {e}", self.port_file.display());
        }
    }
}

/// `DATABASE_URL` with any password replaced, safe to log.
pub fn redact_url(url: &str) -> String {
    let Some((scheme, rest)) = url.split_once("://") else {
        return url.to_string();
    };
    let Some((credentials, host)) = rest.rsplit_once('@') else {
        return url.to_string();
    };
    match credentials.split_once(':') {
        Some((user, _)) => format!("{scheme}://{user}:***@{host}"),
        None => url.to_string(),
    }
}

/// Comma-separated origins, trimmed, empty entries and trailing slashes dropped.
fn parse_origins(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|o| o.trim().trim_end_matches('/'))
        .filter(|o| !o.is_empty())
        .map(str::to_string)
        .collect()
}

fn parse_bool(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" | "" => Some(false),
        _ => None,
    }
}

fn env_bool(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .and_then(|v| parse_bool(&v))
        .unwrap_or(default)
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::{parse_bool, parse_origins, redact_url};

    #[test]
    fn password_is_redacted() {
        assert_eq!(
            redact_url("postgres://postgres:secret@localhost:5432/ax"),
            "postgres://postgres:***@localhost:5432/ax"
        );
        assert_eq!(
            redact_url("postgres://u:p@ss@db/ax"),
            "postgres://u:***@db/ax"
        );
        assert_eq!(
            redact_url("postgres://localhost/ax"),
            "postgres://localhost/ax"
        );
        assert_eq!(redact_url("postgres://user@db/ax"), "postgres://user@db/ax");
    }

    #[test]
    fn origins_are_parsed() {
        assert_eq!(
            parse_origins(" https://a.example/, ,http://localhost:5173"),
            vec!["https://a.example", "http://localhost:5173"]
        );
        assert!(parse_origins("").is_empty());
    }

    #[test]
    fn booleans_are_parsed() {
        assert_eq!(parse_bool("TRUE"), Some(true));
        assert_eq!(parse_bool(" 1 "), Some(true));
        assert_eq!(parse_bool("off"), Some(false));
        assert_eq!(parse_bool("maybe"), None);
    }
}
