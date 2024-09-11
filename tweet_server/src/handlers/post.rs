use std::collections::HashMap;

use actix_web::{web, HttpResponse};

use crate::dbaccess::post::*;
use crate::errors::AxError;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;
use crate::models::post::{CreatePost, UpdatePost};
use crate::state::AppState;

// Create
/*
curl -X POST localhost:8000/posts \
   -H "Content-Type: application/json" \
   -d '{
       "content": "Never Settle",
       "userId": 1,
       "replyTo": null,
       "userName": "John Doe",
       "reactions": null
   }'
*/
pub async fn post_new_post(
    app_state: web::Data<AppState>,
    new_post: web::Json<CreatePost>,
) -> Result<HttpResponse, AxError> {
    insert_post_db(&app_state.db, new_post.into())
        .await
        .map(|post| HttpResponse::Ok().json(post))
}
// Read
/*
curl -X GET http://localhost:8000/posts/1
*/
pub async fn get_post_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (post_id,) = path.into_inner();
    get_post_detail_db(&app_state.db, post_id)
        .await
        .map(|resp| HttpResponse::Ok().json(resp))
}
/*
curl -X GET http://localhost:8000/posts
*/
pub async fn get_post_list(
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    get_post_list_db(&app_state.db, query).await.map(|resp| {
        let api_response = ApiResponse::new(
            200,
            "Success".to_string(),
            Some(
                DataBuilder::new()
                    .set_data(resp.0)
                    .set_pagination(resp.1)
                    .build(),
            ),
        );
        HttpResponse::Ok().json(api_response)
    })
}
// Update
/*
curl -X PUT localhost:8000/posts/1 \
   -H "Content-Type: application/json" \
   -d '{
       "content": "Modified content."
   }'
*/
pub async fn update_post_details(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    update_post: web::Json<UpdatePost>,
) -> Result<HttpResponse, AxError> {
    let (post_id,) = path.into_inner();
    update_post_db(&app_state.db, post_id, update_post.into())
        .await
        .map(|post| HttpResponse::Ok().json(post))
}
// Delete
/*
curl -X DELETE http://localhost:8000/posts/1
*/
pub async fn delete_post(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (post_id,) = path.into_inner();
    delete_post_db(&app_state.db, post_id)
        .await
        .map(|post| HttpResponse::Ok().json(post))
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        handlers::post::{
            delete_post, get_post_detail, insert_post_db, post_new_post, update_post_details,
        },
        models::post::{CreatePost, UpdatePost},
        state::AppState,
    };
    use actix_web::{http::StatusCode, web, ResponseError};
    use dotenv::dotenv;
    use serde_json::Value;
    use sqlx::PgPool;

    #[actix_rt::test]
    async fn test_insert_post() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        let new_post_msg = CreatePost {
            content: "测试内容".to_string(),
            user_id: 1,                             // 假设 1 是一个有效的用户ID
            reply_to: None,                         // 如果没有回复目标则为 None
            reactions: Some(serde_json::json!({})), // 空的 JSON 对象表示没有反应
            user_name: "测试用户".to_string(),
        };
        let post_param = web::Json(new_post_msg.clone());
        let resp = post_new_post(app_state.clone(), post_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // 获取响应 body 中的 JSON 数据
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

        // 从 JSON 中获取 id 字段
        let post_id = body_json["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        // 删除测试插入的 post
        sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_post() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        let post = CreatePost {
            content: String::from("new post"),
            user_id: 1,
            reply_to: None,
            user_name: String::from("mike"),
            reactions: Some(serde_json::json!({})),
        };
        let insert_result = insert_post_db(&app_state.db, post.clone()).await.unwrap();
        assert_eq!("new post", &insert_result.content);
        // Delete test post.
        let delete_params: web::Path<(i32,)> = web::Path::from((insert_result.id,));
        let resp = delete_post(app_state.clone(), delete_params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_post() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        let post = CreatePost {
            content: String::from("test_update_post_before"),
            user_id: 1,
            reply_to: None,
            user_name: String::from("mike"),
            reactions: Some(serde_json::json!({})),
        };
        let insert_result = insert_post_db(&app_state.db, post.clone()).await.unwrap();
        assert_eq!(&post.content, &insert_result.content);
        // Update test user.
        let update_post_msg = UpdatePost {
            content: Some(String::from("test_update_post_after")),
        };
        let parameters: web::Path<(i32,)> = web::Path::from((insert_result.id,));
        let update_param = web::Json(update_post_msg);
        let resp = update_post_details(app_state.clone(), parameters, update_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        // Delete test user.
        sqlx::query!("delete from posts where id = $1", insert_result.id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
    #[actix_rt::test]
    async fn test_get_post_detail() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        let new_post_msg = CreatePost {
            content: "测试内容".to_string(),
            user_id: 1,                             // 假设 1 是一个有效的用户ID
            reply_to: None,                         // 如果没有回复目标则为 None
            reactions: Some(serde_json::json!({})), // 空的 JSON 对象表示没有反应
            user_name: "测试用户".to_string(),
        };
        let result = insert_post_db(&app_state.db, new_post_msg.clone())
            .await
            .unwrap();
        assert_eq!(&new_post_msg.content, &result.content);
        let parameters: web::Path<(i32,)> = web::Path::from((result.id,));
        let resp = get_post_detail(app_state.clone(), parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }

        // Delete test user.
        sqlx::query!("delete from posts where id = $1", result.id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
