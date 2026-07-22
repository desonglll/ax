use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::notification::{
        list_notifications_db, mark_all_read_db, mark_read_db, unread_count_db,
    },
    errors::AxError,
    extractors::{api_response::ApiResponse, data::DataBuilder},
    handlers::auth::login_in_unauthentic,
    state::AppState,
};

/// Paginated list of the caller's notifications.
pub async fn get_notifications(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    let map = query.map(|q| q.into_inner()).unwrap_or_default();
    let limit = map.get("limit").and_then(|s| s.parse().ok()).unwrap_or(20);
    let offset = map.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0);
    let (items, pagination) = list_notifications_db(&app_state.db, user_id, limit, offset).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(
            DataBuilder::new()
                .set_data(items)
                .set_pagination(pagination)
                .build(),
        ),
    )))
}

/// Unread notification count for the caller (for the navbar badge).
pub async fn get_unread_count(
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    let count = unread_count_db(&app_state.db, user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(DataBuilder::new().set_data(count).build()),
    )))
}

/// Mark a single notification read.
pub async fn mark_notification_read(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i64,)>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    let (notification_id,) = path.into_inner();
    let updated = mark_read_db(&app_state.db, user_id, notification_id).await?;
    if updated == 0 {
        return Err(AxError::NotFound("Notification not found".to_string()));
    }
    Ok(HttpResponse::Ok().json(ApiResponse::<()>::new(200, "Marked read".to_string(), None)))
}

/// Mark all of the caller's notifications read.
pub async fn mark_all_notifications_read(
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    let updated = mark_all_read_db(&app_state.db, user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        format!("Marked {} notifications read", updated),
        Some(DataBuilder::new().set_data(updated).build()),
    )))
}

#[cfg(test)]
mod tests {
    use crate::dbaccess::follow::follow_user_db;
    use crate::dbaccess::notification::{list_notifications_db, mark_all_read_db, unread_count_db};
    use crate::dbaccess::user::insert_user_db;
    use crate::models::user::CreateUser;
    use crate::state::get_demo_state;

    fn test_user(name: &str) -> CreateUser {
        CreateUser {
            user_name: name.to_owned(),
            email: format!("{}@test.io", name),
            password: "07001107001100".to_string(),
            full_name: None,
            phone: None,
            is_active: Some(true),
            is_admin: Some(false),
            profile_picture: None,
        }
    }

    #[actix_rt::test]
    async fn test_follow_generates_notification() {
        let app_state = get_demo_state().await;
        let fan = insert_user_db(&app_state.db, test_user("notif_fan"))
            .await
            .unwrap();
        let star = insert_user_db(&app_state.db, test_user("notif_star"))
            .await
            .unwrap();

        assert_eq!(unread_count_db(&app_state.db, star.id).await.unwrap(), 0);
        follow_user_db(&app_state.db, fan.id, star.id)
            .await
            .unwrap();

        assert_eq!(unread_count_db(&app_state.db, star.id).await.unwrap(), 1);
        let (items, pagination) = list_notifications_db(&app_state.db, star.id, 20, 0)
            .await
            .unwrap();
        assert_eq!(pagination.count, Some(1));
        assert_eq!(items[0].kind, "follow");
        assert_eq!(items[0].actor_id, fan.id);
        assert_eq!(items[0].actor_name.as_deref(), Some("notif_fan"));

        let updated = mark_all_read_db(&app_state.db, star.id).await.unwrap();
        assert_eq!(updated, 1);
        assert_eq!(unread_count_db(&app_state.db, star.id).await.unwrap(), 0);

        for id in [fan.id, star.id] {
            sqlx::query!("delete from users where id = $1", id)
                .execute(&app_state.db)
                .await
                .unwrap();
        }
    }

    #[actix_rt::test]
    async fn test_comment_notifies_post_owner_not_self() {
        let app_state = get_demo_state().await;
        let author = insert_user_db(&app_state.db, test_user("notif_author"))
            .await
            .unwrap();
        let commenter = insert_user_db(&app_state.db, test_user("notif_commenter"))
            .await
            .unwrap();

        let post_id: uuid::Uuid = sqlx::query_scalar!(
            "insert into posts (title, content, user_id) values ('t', 'c', $1) returning id",
            author.id
        )
        .fetch_one(&app_state.db)
        .await
        .unwrap();

        // Self-comment: no notification.
        sqlx::query!(
            "insert into comments (content, reply_to, user_id) values ('mine', $1, $2)",
            post_id,
            author.id
        )
        .execute(&app_state.db)
        .await
        .unwrap();
        assert_eq!(unread_count_db(&app_state.db, author.id).await.unwrap(), 0);

        // Someone else's comment: one notification for the author.
        sqlx::query!(
            "insert into comments (content, reply_to, user_id) values ('hi', $1, $2)",
            post_id,
            commenter.id
        )
        .execute(&app_state.db)
        .await
        .unwrap();
        assert_eq!(unread_count_db(&app_state.db, author.id).await.unwrap(), 1);
        let (items, _) = list_notifications_db(&app_state.db, author.id, 20, 0)
            .await
            .unwrap();
        assert_eq!(items[0].kind, "comment");
        assert_eq!(items[0].post_id, Some(post_id));

        for id in [author.id, commenter.id] {
            sqlx::query!("delete from users where id = $1", id)
                .execute(&app_state.db)
                .await
                .unwrap();
        }
    }
}
