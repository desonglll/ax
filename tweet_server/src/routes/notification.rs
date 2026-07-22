use actix_web::web;

use crate::handlers::notification::{
    get_notifications, get_unread_count, mark_all_notifications_read, mark_notification_read,
};

/// Configure routes related to notifications.
pub fn notification_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/notifications")
            .route("/get", web::get().to(get_notifications))
            .route("/unread-count", web::get().to(get_unread_count))
            .route(
                "/read/{notification_id}",
                web::post().to(mark_notification_read),
            )
            .route("/read-all", web::post().to(mark_all_notifications_read)),
    );
}
