use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::reaction::*;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;
use crate::{errors::AxError, models::reaction::CreateReaction, state::AppState};

// Create
/*
http://localhost:8000/api/reactions/post/like?userId=2&postId=1
*/
pub async fn post_like_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query = query.unwrap();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let post_id = query
        .get("postId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let new_reaction = CreateReaction { user_id, post_id };
    println!("{:#?}", new_reaction);
    insert_like_reaction_db(&app_state.db, new_reaction)
        .await
        .map(|reaction| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Insert Like Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction).build()),
            ))
        })
}

/*
http://localhost:8000/api/reactions/post/dislike?userId=2&postId=1
*/
pub async fn post_dislike_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query = query.unwrap();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let post_id = query
        .get("postId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let new_reaction = CreateReaction { user_id, post_id };
    insert_dislike_reaction_db(&app_state.db, new_reaction)
        .await
        .map(|reaction| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Insert Dislike Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction).build()),
            ))
        })
}
// Read
/*
http://localhost:8000/api/reactions/get?postId=1
 */
pub async fn get_reaction_by_post_id(
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let post_id = query
        .unwrap()
        .get("postId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap();
    get_reaction_by_post_id_db(&app_state.db, post_id)
        .await
        .map(|reaction_table| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get Reaction Table Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction_table).build()),
            ))
        })
}

pub async fn get_reaction_by_user_id_and_post_id(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let post_id = query
        .unwrap()
        .get("postId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    get_reaction_by_user_id_and_post_id_db(&app_state.db, post_id, user_id)
        .await
        .map(|reaction| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get Reaction Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction).build()),
            ))
        })
}
// Delete
pub async fn delete_reaction_by_id(
    _session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let reaction_id = query
        .unwrap()
        .get("reactionId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    Ok(delete_reaction_by_id_db(&app_state.db, reaction_id)
        .await
        .map(|reaction| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Delete Reaction Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction).build()),
            ))
        })?)
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        handlers::reaction::{post_dislike_reaction, post_like_reaction},
        state::AppState,
    };
    use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
    use actix_web::{cookie::Key, http::StatusCode, test, web, App};
    use dotenv::dotenv;
    use serde_json::Value;
    use sqlx::PgPool;

    #[actix_rt::test]
    async fn test_insert_like_reaction() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });

        // 模拟请求和会话
        let secret_key = Key::generate();
        let redis_connection_string = "127.0.0.1:6379";
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(
                        RedisActorSessionStore::new(redis_connection_string),
                        secret_key.clone(),
                    )
                    .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                    .build(),
                )
                .route("/like", web::post().to(post_like_reaction)),
        )
        .await;

        // 构建请求，并模拟会话
        let req = test::TestRequest::post()
            .uri("/like?postId=1") // 设置 query 参数
            .cookie(actix_web::cookie::Cookie::new("user_id", "1")) // 设置 cookie 模拟 session
            .to_request();

        // 发送请求
        let resp = test::call_service(&app, req).await;

        // 检查响应状态码
        assert_eq!(resp.status(), StatusCode::OK);
        // 获取响应 body 中的 JSON 数据
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
        println!("{body_json}");

        // 从 JSON 中获取 id 字段
        let reaction_id = body_json["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        // 删除测试插入的 reaction
        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
    #[actix_rt::test]
    async fn test_insert_dislike_reaction() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        // 模拟请求和会话
        let secret_key = Key::generate();
        let redis_connection_string = "127.0.0.1:6379";
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(
                        RedisActorSessionStore::new(redis_connection_string),
                        secret_key.clone(),
                    )
                    .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                    .build(),
                )
                .route("/dislike", web::post().to(post_dislike_reaction)),
        )
        .await;

        // 构建请求，并模拟会话
        let req = test::TestRequest::post()
            .uri("/dislike?postId=1") // 设置 query 参数
            .cookie(actix_web::cookie::Cookie::new("user_id", "1")) // 设置 cookie 模拟 session
            .to_request();

        // 发送请求
        let resp = test::call_service(&app, req).await;

        // 检查响应状态码
        assert_eq!(resp.status(), StatusCode::OK);

        // 获取响应 body 中的 JSON 数据
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

        // 从 JSON 中获取 id 字段
        let reaction_id = body_json["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        // 删除测试插入的 reaction
        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
