use actix_web::{web, HttpResponse};

use crate::dbaccess::post::*;
use crate::errors::AxError;
use crate::models::post::CreatePost;
use crate::state::AppState;

// Create
/*
curl -X POST localhost:8000/posts \
   -H "Content-Type: application/json" \
   -d '{
       "content": "Never Settle",
       "user_id": 1,
       "reply_to": null,
       "user_name": "John Doe",
       "reactions": null
   }'
*/
pub async fn post_new_post(
    app_state: web::Data<AppState>,
    new_post: web::Json<CreatePost>,
) -> Result<HttpResponse, AxError> {
    insert_post_db(&app_state.db, new_post.into())
        .await
        .map(|user| HttpResponse::Ok().json(user))
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{handlers::post::post_new_post, models::post::CreatePost, state::AppState};
    use actix_web::{http::StatusCode, web};
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
}
