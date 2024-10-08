use std::collections::HashMap;

use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{errors::AxError, models::reaction::CreateReaction, state::AppState};
use crate::dbaccess::reaction::*;
use crate::handlers::auth::login_in_unauthentic;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;

// Create
/*
http://localhost:8000/api/reactions/post/like?userId=2&postId=1
*/
pub async fn insert_like_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query = query.unwrap();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let to_id = query
        .get("toId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let to_type = query.get("toType").unwrap_or(&"post".to_string()).clone();

    let new_reaction = CreateReaction {
        user_id,
        to_id,
        to_type,
    };
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
pub async fn insert_dislike_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query = query.unwrap();
    println!("{:?}", query);
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let to_id = query
        .get("toId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let to_type = query.get("toType").unwrap_or(&"post".to_string()).clone();
    println!("to_type: {:?}", to_type);
    let new_reaction = CreateReaction {
        user_id,
        to_id,
        to_type,
    };
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
pub async fn get_single_reaction_table_by_query(
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    get_reaction_table_by_query_db(&app_state.db, query.unwrap())
        .await
        .map(|reaction_table| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get Reaction Table Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction_table).build()),
            ))
        })
}

pub async fn get_reactions_by_query(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let mut query = query.unwrap();
    if query.get("userId").is_none() {
        let user_id = session.get::<i32>("user_id").unwrap();
        query.insert("userId".to_string(), user_id.unwrap().to_string());
    }
    get_reactions_by_query_db(&app_state.db, query)
        .await
        .map(|reactions| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get Reactions Successful".to_string(),
                Some(DataBuilder::new().set_data(reactions).build()),
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
    use actix_session::SessionMiddleware;
    use actix_session::storage::RedisSessionStore;
    use actix_web::{App, cookie::Key, http::StatusCode, test, web};
    use serde_json::Value;

    use crate::{
        handlers::reaction::{insert_dislike_reaction, insert_like_reaction},
        state::{AppState, get_demo_state},
    };

    #[actix_rt::test]
    async fn test_insert_like_reaction() {
        let app_state: web::Data<AppState> = get_demo_state().await;

        // 模拟请求和会话
        let secret_key = Key::generate();
        let redis_connection_string = "redis://127.0.0.1:6379";
        let store = RedisSessionStore::new(redis_connection_string)
            .await
            .unwrap();

        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                        .build(),
                )
                .route("/like", web::post().to(insert_like_reaction)),
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
        let app_state: web::Data<AppState> = get_demo_state().await;
        // 模拟请求和会话
        let secret_key = Key::generate();
        let redis_connection_string = "redis://127.0.0.1:6379";
        let store = RedisSessionStore::new(redis_connection_string)
            .await
            .unwrap();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false) // https://docs.rs/actix-session/latest/actix_session/config/struct.SessionMiddlewareBuilder.html#method.cookie_secure
                        .build(),
                )
                .route("/dislike", web::post().to(insert_dislike_reaction)),
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
