use actix_web::{web, HttpResponse};

use crate::dbaccess::reaction::*;
use crate::{errors::AxError, models::reaction::CreateReaction, state::AppState};

// Create
/*
curl -X POST localhost:8000/api/reactions/post \
   -H "Content-Type: application/json" \
   -d '{
       "userId": 1,
       "postId": 1,
   }'
*/
pub async fn post_new_reaction(
    app_state: web::Data<AppState>,
    new_reaction: web::Json<CreateReaction>,
) -> Result<HttpResponse, AxError> {
    insert_like_reaction_db(&app_state.db, new_reaction.into())
        .await
        .map(|reaction| HttpResponse::Ok().json(reaction))
}

// Read
pub async fn get_reaction_by_post_id(
    app_state: web::Data<AppState>,
    post_id: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (post_id,) = post_id.into_inner();
    get_reaction_by_post_id_db(&app_state.db, post_id)
        .await
        .map(|reaction_table| HttpResponse::Ok().json(reaction_table))
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::{
        handlers::reaction::post_new_reaction, models::reaction::CreateReaction, state::AppState,
    };
    use actix_web::{http::StatusCode, web};
    use dotenv::dotenv;
    use serde_json::Value;
    use sqlx::PgPool;

    #[actix_rt::test]
    async fn test_insert_like_reaction() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState { db: pool });
        let new_reaction_msg = CreateReaction {
            user_id: 1, // 假设 1 是一个有效的用户ID
            post_id: 1,
        };
        let post_param = web::Json(new_reaction_msg.clone());
        let resp = post_new_reaction(app_state.clone(), post_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // 获取响应 body 中的 JSON 数据
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

        // 从 JSON 中获取 id 字段
        let reaction_id = body_json["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;

        // 删除测试插入的 reaction
        sqlx::query!("DELETE FROM reactions WHERE id = $1", reaction_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
