use std::io::Write;
use std::net::SocketAddr;
use std::path::PathBuf;

/// Runtime server configuration resolved from environment variables.
///
/// - `HOST` (default `0.0.0.0`): interface the HTTP server binds to.
/// - `PORT` (default `8000`): TCP port. `PORT=0` requests an OS-assigned
///   random port, which is useful for running tests or several debug
///   instances side by side.
/// - `PORT_FILE` (default `.server-port` in the working directory): once the
///   server is bound, the actual address is written here so other tools
///   (frontend dev proxy, scripts, tests) can discover the port even when it
///   was randomly assigned.
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub port_file: PathBuf,
}

impl ServerConfig {
    pub fn from_env() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8000);
        let port_file = std::env::var("PORT_FILE")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from(".server-port"));
        Self {
            host,
            port,
            port_file,
        }
    }

    /// Record the actually bound addresses so external tools can find the
    /// server even when `PORT=0` picked a random port.
    ///
    /// Failures are logged but never fatal: the port file is a convenience,
    /// not a requirement for serving traffic.
    pub fn publish_bound_addrs(&self, addrs: &[SocketAddr]) {
        let Some(addr) = addrs.first() else {
            return;
        };
        tracing::info!("server listening on http://{addr}");
        match std::fs::File::create(&self.port_file)
            .and_then(|mut f| writeln!(f, "{}", addr.port()))
        {
            Ok(()) => tracing::info!(
                "wrote bound port {} to {}",
                addr.port(),
                self.port_file.display()
            ),
            Err(e) => tracing::warn!(
                "could not write port file {}: {e}",
                self.port_file.display()
            ),
        }
    }
}

/// Load the session signing key.
///
/// Reads `SESSION_SECRET_KEY` (any string of at least 32 bytes) and derives a
/// stable cookie key from it so sessions survive server restarts. When the
/// variable is absent a random key is generated and a warning is emitted:
/// every restart will then invalidate all existing sessions.
pub fn session_key() -> actix_web::cookie::Key {
    match std::env::var("SESSION_SECRET_KEY") {
        Ok(secret) if secret.len() >= 32 => actix_web::cookie::Key::derive_from(secret.as_bytes()),
        Ok(_) => {
            tracing::warn!(
                "SESSION_SECRET_KEY is shorter than 32 bytes; falling back to a random key \
                 (all sessions reset on restart)"
            );
            actix_web::cookie::Key::generate()
        }
        Err(_) => {
            tracing::warn!(
                "SESSION_SECRET_KEY not set; using a random key (all sessions reset on restart)"
            );
            actix_web::cookie::Key::generate()
        }
    }
}
