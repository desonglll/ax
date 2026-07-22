use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::follow::{
        follow_user_db, get_feed_db, get_follow_stats_db, list_followers_db, list_following_db,
        unfollow_user_db,
    },
    errors::AxError,
    extractors::{api_response::ApiResponse, data::DataBuilder},
    handlers::auth::login_in_unauthentic,
    state::AppState,
};

fn parse_page(query: &Option<web::Query<HashMap<String, String>>>) -> (i64, i64) {
    let map = query.as_ref().map(|q| q.0.clone()).unwrap_or_default();
    let limit = map.get("limit").and_then(|s| s.parse().ok()).unwrap_or(20);
    let offset = map.get("offset").and_then(|s| s.parse().ok()).unwrap_or(0);
    (limit, offset)
}

/// Follow the user in the URL path.
pub async fn follow_user(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let (followee_id,) = path.into_inner();
    let follower_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    if follower_id == followee_id {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::new(
            400,
            "Cannot follow yourself".to_string(),
            None,
        )));
    }
    follow_user_db(&app_state.db, follower_id, followee_id).await?;
    let stats = get_follow_stats_db(&app_state.db, followee_id, Some(follower_id)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Followed".to_string(),
        Some(DataBuilder::new().set_data(stats).build()),
    )))
}

/// Unfollow the user in the URL path.
pub async fn unfollow_user(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let (followee_id,) = path.into_inner();
    let follower_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    unfollow_user_db(&app_state.db, follower_id, followee_id).await?;
    let stats = get_follow_stats_db(&app_state.db, followee_id, Some(follower_id)).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Unfollowed".to_string(),
        Some(DataBuilder::new().set_data(stats).build()),
    )))
}

/// Follower/following counts for a profile; `isFollowing` reflects the caller.
pub async fn get_follow_stats(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    let viewer_id = session.get::<i32>("user_id").ok().flatten();
    let stats = get_follow_stats_db(&app_state.db, user_id, viewer_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(DataBuilder::new().set_data(stats).build()),
    )))
}

/// Paginated list of a user's followers.
pub async fn get_followers(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    let (limit, offset) = parse_page(&query);
    let (users, pagination) = list_followers_db(&app_state.db, user_id, limit, offset).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(
            DataBuilder::new()
                .set_data(users)
                .set_pagination(pagination)
                .build(),
        ),
    )))
}

/// Paginated list of users someone follows.
pub async fn get_following(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    let (limit, offset) = parse_page(&query);
    let (users, pagination) = list_following_db(&app_state.db, user_id, limit, offset).await?;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(
            DataBuilder::new()
                .set_data(users)
                .set_pagination(pagination)
                .build(),
        ),
    )))
}

/// Personalized timeline: posts from users the caller follows, with
/// attachments, newest first.
pub async fn get_feed(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let viewer_id = session.get::<i32>("user_id").ok().flatten().unwrap_or(0);
    let (limit, offset) = parse_page(&query);
    let (posts, pagination) = get_feed_db(&app_state.db, viewer_id, limit, offset).await?;
    let posts_with_files = super::post::attach_files_to_posts(&app_state.db, posts).await;
    Ok(HttpResponse::Ok().json(ApiResponse::new(
        200,
        "Success".to_string(),
        Some(
            crate::extractors::data::PostListDataBuilder::new()
                .set_data(posts_with_files)
                .set_pagination(pagination)
                .build(),
        ),
    )))
}

#[cfg(test)]
mod tests {
    use actix_web::{http::StatusCode, web};
    use serde_json::Value;

    use crate::dbaccess::follow::{
        follow_user_db, get_feed_db, get_follow_stats_db, unfollow_user_db,
    };
    use crate::dbaccess::user::insert_user_db;
    use crate::models::user::CreateUser;
    use crate::state::get_demo_state;
    use crate::utils::test::{get_test_session, http_response_to_json};

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
    async fn test_follow_unfollow_and_stats() {
        let app_state = get_demo_state().await;
        let alice = insert_user_db(&app_state.db, test_user("follow_alice"))
            .await
            .unwrap();
        let bob = insert_user_db(&app_state.db, test_user("follow_bob"))
            .await
            .unwrap();

        follow_user_db(&app_state.db, alice.id, bob.id)
            .await
            .unwrap();
        // Idempotent double-follow must not error.
        follow_user_db(&app_state.db, alice.id, bob.id)
            .await
            .unwrap();

        let stats = get_follow_stats_db(&app_state.db, bob.id, Some(alice.id))
            .await
            .unwrap();
        assert_eq!(stats.followers_count, 1);
        assert!(stats.is_following);

        unfollow_user_db(&app_state.db, alice.id, bob.id)
            .await
            .unwrap();
        let stats = get_follow_stats_db(&app_state.db, bob.id, Some(alice.id))
            .await
            .unwrap();
        assert_eq!(stats.followers_count, 0);
        assert!(!stats.is_following);

        for id in [alice.id, bob.id] {
            sqlx::query!("delete from users where id = $1", id)
                .execute(&app_state.db)
                .await
                .unwrap();
        }
    }

    #[actix_rt::test]
    async fn test_feed_contains_followed_users_posts() {
        let app_state = get_demo_state().await;
        let reader = insert_user_db(&app_state.db, test_user("feed_reader"))
            .await
            .unwrap();
        let author = insert_user_db(&app_state.db, test_user("feed_author"))
            .await
            .unwrap();

        sqlx::query!(
            "insert into posts (title, content, user_id) values ('t', 'feed post content', $1)",
            author.id
        )
        .execute(&app_state.db)
        .await
        .unwrap();

        // Before following: empty feed.
        let (posts, _) = get_feed_db(&app_state.db, reader.id, 20, 0).await.unwrap();
        assert!(posts.is_empty());

        follow_user_db(&app_state.db, reader.id, author.id)
            .await
            .unwrap();
        let (posts, pagination) = get_feed_db(&app_state.db, reader.id, 20, 0).await.unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(pagination.count, Some(1));
        assert_eq!(posts[0].content, "feed post content");

        // users cascade-deletes follows and posts.
        for id in [reader.id, author.id] {
            sqlx::query!("delete from users where id = $1", id)
                .execute(&app_state.db)
                .await
                .unwrap();
        }
    }

    #[actix_rt::test]
    async fn test_cannot_follow_self() {
        let app_state = get_demo_state().await;
        let user = insert_user_db(&app_state.db, test_user("follow_self"))
            .await
            .unwrap();
        let session = get_test_session(&user).await;
        session.insert("is_active", true).unwrap();
        let path: web::Path<(i32,)> = web::Path::from((user.id,));
        let resp = super::follow_user(session, app_state.clone(), path)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body: Value = http_response_to_json(resp).await;
        assert_eq!(body["code"], 400);
        sqlx::query!("delete from users where id = $1", user.id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
