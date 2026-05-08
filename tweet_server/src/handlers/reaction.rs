use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::reaction::*;
use crate::handlers::auth::login_in_unauthentic;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;
use crate::{errors::AxError, models::reaction::CreateReaction, state::AppState};

// Create
/*
http://localhost:8000/api/reactions/post/like?userId=2&postId=1
*/
/// 点赞
///
/// 为指定目标（推文或评论）添加点赞互动。从 session 获取当前用户 ID，
/// 从查询参数获取目标 ID 和类型。如果之前已点踩，会先删除点踩记录。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于获取用户 ID
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数，支持 `toId` 和 `toType` 字段
///
/// # 返回值
///
/// 成功时返回 200 响应及点赞记录，失败时返回 [`AxError`]。
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
/// 点踩
///
/// 为指定目标（推文或评论）添加点踩互动。从 session 获取当前用户 ID，
/// 从查询参数获取目标 ID 和类型。如果之前已点赞，会先删除点赞记录。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于获取用户 ID
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数，支持 `toId` 和 `toType` 字段
///
/// # 返回值
///
/// 成功时返回 200 响应及点踩记录，失败时返回 [`AxError`]。
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
/// 获取指定目标的互动统计表
///
/// 根据查询参数中的目标 ID，返回该目标的点赞和点踩数量统计。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数，支持 `toId` 字段
///
/// # 返回值
///
/// 成功时返回 200 响应及互动统计表（like 和 dislike 计数），失败时返回 [`AxError`]。
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

/// 根据查询条件获取互动记录列表
///
/// 验证登录状态后，根据 URL 查询参数筛选互动记录。
/// 如果查询参数中未提供 `userId`，则自动使用当前 session 中的用户 ID。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证和获取用户 ID
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数，支持 `id`、`toId`、`toType`、`userId`、`reactionName`
///
/// # 返回值
///
/// 成功时返回 200 响应及互动记录列表，失败时返回 [`AxError`]。
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
/// 根据互动 ID 删除互动记录
///
/// 根据查询参数中的 `reactionId` 删除指定的互动记录。
///
/// # 参数
///
/// - `_session`: 请求的 session 对象（暂未使用）
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数，支持 `reactionId` 字段
///
/// # 返回值
///
/// 成功时返回 200 响应及被删除的互动记录，失败时返回 [`AxError`]。
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
    use actix_session::storage::RedisSessionStore;
    use actix_session::SessionMiddleware;
    use actix_web::{cookie::Key, http::StatusCode, test, web, App};
    use serde_json::Value;

    use crate::{
        handlers::reaction::{
            delete_reaction_by_id, get_single_reaction_table_by_query, insert_dislike_reaction,
            insert_like_reaction,
        },
        state::{get_demo_state, AppState},
    };

    #[actix_rt::test]
    async fn test_insert_like_reaction() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let secret_key = Key::generate();
        let store = RedisSessionStore::new("redis://127.0.0.1:6379")
            .await
            .unwrap();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false)
                        .build(),
                )
                .route("/like", web::post().to(insert_like_reaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/like?toId=1&toType=post")
            .cookie(actix_web::cookie::Cookie::new("user_id", "1"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Value = test::read_body_json(resp).await;
        let reaction_id = body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_insert_dislike_reaction() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let secret_key = Key::generate();
        let store = RedisSessionStore::new("redis://127.0.0.1:6379")
            .await
            .unwrap();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false)
                        .build(),
                )
                .route("/dislike", web::post().to(insert_dislike_reaction)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/dislike?toId=1&toType=post")
            .cookie(actix_web::cookie::Cookie::new("user_id", "1"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Value = test::read_body_json(resp).await;
        let reaction_id = body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_get_reaction_table() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let secret_key = Key::generate();
        let store = RedisSessionStore::new("redis://127.0.0.1:6379")
            .await
            .unwrap();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false)
                        .build(),
                )
                .route("/like", web::post().to(insert_like_reaction))
                .route("/table", web::get().to(get_single_reaction_table_by_query)),
        )
        .await;

        // First insert a like
        let insert_req = test::TestRequest::post()
            .uri("/like?toId=9999&toType=post")
            .cookie(actix_web::cookie::Cookie::new("user_id", "1"))
            .to_request();
        let insert_resp = test::call_service(&app, insert_req).await;
        assert_eq!(insert_resp.status(), StatusCode::OK);
        let insert_body: Value = test::read_body_json(insert_resp).await;
        let reaction_id = insert_body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found") as i32;

        // Then get the reaction table
        let get_req = test::TestRequest::get()
            .uri("/table?toId=9999&toType=post")
            .to_request();
        let get_resp = test::call_service(&app, get_req).await;
        assert_eq!(get_resp.status(), StatusCode::OK);
        let get_body: Value = test::read_body_json(get_resp).await;
        assert_eq!(get_body["code"], 200);

        // Cleanup
        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_reaction() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let secret_key = Key::generate();
        let store = RedisSessionStore::new("redis://127.0.0.1:6379")
            .await
            .unwrap();
        let app = test::init_service(
            App::new()
                .app_data(app_state.clone())
                .wrap(
                    SessionMiddleware::builder(store, secret_key.clone())
                        .cookie_secure(false)
                        .build(),
                )
                .route("/like", web::post().to(insert_like_reaction))
                .route("/delete", web::delete().to(delete_reaction_by_id)),
        )
        .await;

        // First insert a like
        let insert_req = test::TestRequest::post()
            .uri("/like?toId=8888&toType=post")
            .cookie(actix_web::cookie::Cookie::new("user_id", "1"))
            .to_request();
        let insert_resp = test::call_service(&app, insert_req).await;
        let insert_body: Value = test::read_body_json(insert_resp).await;
        let reaction_id = insert_body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found") as i32;

        // Delete the reaction
        let delete_req = test::TestRequest::delete()
            .uri(format!("/delete?reactionId={}", reaction_id).as_str())
            .to_request();
        let delete_resp = test::call_service(&app, delete_req).await;
        assert_eq!(delete_resp.status(), StatusCode::OK);
        let delete_body: Value = test::read_body_json(delete_resp).await;
        assert_eq!(delete_body["code"], 200);
    }
}
