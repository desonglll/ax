//! Cross-cutting HTTP concerns: security headers, request ids, CORS, the
//! rate limiter's client key.

use std::net::IpAddr;

use actix_cors::Cors;
use actix_governor::{KeyExtractor, SimpleKeyExtractionError};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    middleware::DefaultHeaders,
};
use uuid::Uuid;

pub const REQUEST_ID: &str = "x-request-id";

/// Headers added to every response unless a handler set them already. The
/// API only ever returns JSON or file bodies, so its CSP can deny everything;
/// `sandbox` also neutralises an uploaded HTML/SVG file opened directly.
pub fn security_headers() -> DefaultHeaders {
    DefaultHeaders::new()
        .add(("X-Content-Type-Options", "nosniff"))
        .add(("X-Frame-Options", "DENY"))
        .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
        .add((
            "Content-Security-Policy",
            "default-src 'none'; frame-ancestors 'none'; sandbox",
        ))
}

/// `None` keeps the permissive development default; a list restricts CORS
/// (with credentials) to exactly those origins.
pub fn cors(origins: Option<&[String]>) -> Cors {
    match origins {
        None => Cors::permissive(),
        Some(origins) => origins
            .iter()
            .fold(Cors::default(), |cors, origin| cors.allowed_origin(origin))
            .allow_any_method()
            .allow_any_header()
            .expose_headers([REQUEST_ID])
            .supports_credentials()
            .max_age(3600),
    }
}

/// Keeps a well-formed incoming `X-Request-Id` (so ids can be traced through
/// a proxy), otherwise generates one.
pub fn request_id(incoming: Option<&str>) -> String {
    match incoming {
        Some(id) if is_valid_request_id(id) => id.to_string(),
        _ => Uuid::new_v4().simple().to_string(),
    }
}

fn is_valid_request_id(id: &str) -> bool {
    (1..=64).contains(&id.len())
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.')
}

/// `wrap_fn` body: tags the request and its response with an `X-Request-Id`.
pub fn with_request_id<S, B>(
    mut req: ServiceRequest,
    srv: &S,
) -> impl std::future::Future<Output = Result<ServiceResponse<B>, actix_web::Error>>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = actix_web::Error>,
{
    let id = request_id(req.headers().get(REQUEST_ID).and_then(|v| v.to_str().ok()));
    // Validated/generated ids are always valid header values.
    let value = HeaderValue::from_str(&id).unwrap_or_else(|_| HeaderValue::from_static("invalid"));
    req.headers_mut()
        .insert(HeaderName::from_static(REQUEST_ID), value.clone());
    let response = srv.call(req);
    async move {
        let mut response = response.await?;
        response
            .headers_mut()
            .insert(HeaderName::from_static(REQUEST_ID), value);
        Ok(response)
    }
}

/// Rate-limit key: the client's IP address. Behind a reverse proxy
/// (`trust_proxy`) the peer is always the proxy, so the address comes from
/// `X-Real-IP`, else the right-most `X-Forwarded-For` entry (the one the
/// proxy appended; left-most entries are client-controlled).
#[derive(Debug, Clone, Copy)]
pub struct ClientIpKeyExtractor {
    pub trust_proxy: bool,
}

impl KeyExtractor for ClientIpKeyExtractor {
    type Key = IpAddr;
    type KeyExtractionError = SimpleKeyExtractionError<&'static str>;

    fn extract(&self, req: &ServiceRequest) -> Result<Self::Key, Self::KeyExtractionError> {
        let header = |name: &str| req.headers().get(name).and_then(|v| v.to_str().ok());
        let peer = req.peer_addr().map(|a| a.ip());
        let ip = if self.trust_proxy {
            client_ip(header("x-real-ip"), header("x-forwarded-for"), peer)
        } else {
            peer
        };
        ip.map(rate_limit_bucket).ok_or_else(|| {
            SimpleKeyExtractionError::new("Could not determine the client IP address")
        })
    }
}

fn client_ip(
    real_ip: Option<&str>,
    forwarded_for: Option<&str>,
    peer: Option<IpAddr>,
) -> Option<IpAddr> {
    let parse = |v: &str| v.trim().parse::<IpAddr>().ok();
    real_ip
        .and_then(parse)
        .or_else(|| {
            forwarded_for
                .and_then(|v| v.rsplit(',').next())
                .and_then(parse)
        })
        .or(peer)
}

/// IPv6 users usually own a whole /56, so limit per prefix rather than per
/// address (as actix-governor's own peer extractor does).
fn rate_limit_bucket(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V6(v6) => {
            let mut octets = v6.octets();
            octets[7..].fill(0);
            IpAddr::V6(octets.into())
        }
        v4 => v4,
    }
}

#[cfg(test)]
mod tests {
    use std::net::IpAddr;

    use super::{client_ip, is_valid_request_id, rate_limit_bucket, request_id};

    #[test]
    fn client_ip_prefers_proxy_headers() {
        let peer: Option<IpAddr> = "10.0.0.1".parse().ok();
        let ip = |s: &str| s.parse::<IpAddr>().ok();
        assert_eq!(
            client_ip(Some("1.2.3.4"), Some("9.9.9.9"), peer),
            ip("1.2.3.4")
        );
        assert_eq!(
            client_ip(None, Some("6.6.6.6, 5.5.5.5"), peer),
            ip("5.5.5.5"),
            "the right-most entry is the one the proxy added"
        );
        assert_eq!(client_ip(Some("garbage"), None, peer), peer);
        assert_eq!(client_ip(None, None, None), None);
    }

    #[test]
    fn ipv6_is_bucketed_per_prefix() {
        let a: IpAddr = "2001:db8:1:2:3:4:5:6".parse().unwrap();
        let b: IpAddr = "2001:db8:1:2:ff:4:5:6".parse().unwrap();
        assert_eq!(rate_limit_bucket(a), rate_limit_bucket(b));
        let v4: IpAddr = "1.2.3.4".parse().unwrap();
        assert_eq!(rate_limit_bucket(v4), v4);
    }

    #[test]
    fn incoming_request_ids_are_validated() {
        assert_eq!(request_id(Some("abc-123_x.y")), "abc-123_x.y");
        assert_ne!(request_id(Some("bad id")), "bad id");
        assert_ne!(request_id(Some("")), "");
        assert_eq!(request_id(None).len(), 32);
        assert!(!is_valid_request_id(&"a".repeat(65)));
        assert!(!is_valid_request_id("evil\r\nheader"));
    }
}
