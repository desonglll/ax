use actix_web::web;

use crate::handlers::auth::{login, logout};
use crate::handlers::user::get_user_profile;

/// Configure routes related to authentication.
pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            // Returns the logged-in user's profile so clients can restore
            // their session state (the previous handler returned no data).
            .route("/login-check", web::get().to(get_user_profile))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout)),
    );
}
