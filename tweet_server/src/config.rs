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
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub port_file: PathBuf,
    pub upload_dir: PathBuf,
    pub rate_per_second: u64,
    pub rate_burst: u32,
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

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_parse<T: std::str::FromStr>(key: &str, default: T) -> T {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
