//! The URL table. Everything is under `/api`.

use actix_web::web::{self, delete, get, post, put};

use crate::handlers::{
    auth, comment, file, follow, notification, post as posts, reaction, upload, user,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/health", get().to(health))
            .service(
                web::scope("/auth")
                    .route("/login", post().to(auth::login))
                    .route("/logout", post().to(auth::logout))
                    .route("/me", get().to(auth::me)),
            )
            .service(
                web::scope("/users")
                    .route("", get().to(user::list))
                    .route("", post().to(user::register))
                    .route("/{id}", get().to(user::get))
                    .route("/{id}", put().to(user::update))
                    .route("/{id}", delete().to(user::remove))
                    .route("/{id}/follow", post().to(follow::follow))
                    .route("/{id}/follow", delete().to(follow::unfollow))
                    .route("/{id}/follow-stats", get().to(follow::stats))
                    .route("/{id}/followers", get().to(follow::followers))
                    .route("/{id}/following", get().to(follow::following)),
            )
            .service(
                web::scope("/posts")
                    .route("", get().to(posts::list))
                    .route("", post().to(posts::create))
                    .route("/feed", get().to(posts::feed))
                    .route("/trending", get().to(posts::trending))
                    .route("/{id}", get().to(posts::get))
                    .route("/{id}", put().to(posts::update))
                    .route("/{id}", delete().to(posts::remove)),
            )
            .service(
                web::scope("/comments")
                    .route("", get().to(comment::list))
                    .route("", post().to(comment::create))
                    .route("/{id}", delete().to(comment::remove)),
            )
            .service(
                web::scope("/reactions")
                    .route("", put().to(reaction::set))
                    .route("", delete().to(reaction::remove)),
            )
            .service(
                web::scope("/notifications")
                    .route("", get().to(notification::list))
                    .route("/unread-count", get().to(notification::unread_count))
                    .route("/read-all", post().to(notification::mark_all_read))
                    .route("/{id}/read", post().to(notification::mark_read)),
            )
            .service(
                web::scope("/files")
                    .route("", get().to(file::list))
                    .route("", post().to(upload::upload))
                    .route("/{id}/download", get().to(file::download))
                    .route("/{id}/stream", get().to(file::stream)),
            ),
    );
}

async fn health() -> actix_web::HttpResponse {
    crate::response::ok_message("ok")
}
