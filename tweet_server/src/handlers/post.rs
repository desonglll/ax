use std::collections::HashMap;

use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::dbaccess::post::*;
use crate::errors::AxError;
use crate::handlers::auth::login_in_unauthentic;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::{DataBuilder, PostListDataBuilder};
use crate::libraries::session::SessionOperation;
use crate::models::post::{CreatePost, UpdatePost};
use crate::services::recommend::post::recommend_posts;
use crate::state::AppState;

// Create
/*
curl -X POST localhost:8000/api/posts/post \
   -H "Content-Type: application/json" \
   -d '{
       "content": "Never Settle",
       "userId": 1,
       "replyTo": null,
       "userName": "John Doe",
       "reactions": null
   }'
*/
pub async fn insert_new_post(
    session: Session,
    app_state: web::Data<AppState>,
    new_post: web::Json<CreatePost>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let _ = login_in_unauthentic(&session).await;
    let mut new_post: CreatePost = new_post.into();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap();
    new_post.set_user_id(user_id);

    insert_post_db(&app_state.db, new_post).await.map(|post| {
        let api_response = ApiResponse::new(
            200,
            "Insert Post Successful".to_string(),
            Some(DataBuilder::new().set_data(post).build()),
        );
        HttpResponse::Ok().json(api_response)
    })
}

// Read
/*
curl -X GET http://localhost:8000/api/posts/get/1
*/
pub async fn get_post_detail(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let _ = login_in_unauthentic(&session).await;
    let (post_id, ) = path.into_inner();
    get_post_detail_db(&app_state.db, post_id)
        .await
        .map(|resp| {
            let api_response = ApiResponse::new(
                200,
                "Get Post Successful".to_string(),
                Some(DataBuilder::new().set_data(resp).build()),
            );
            HttpResponse::Ok().json(api_response)
        })
}
/*
curl -X GET http://localhost:8000/api/posts/get
*/
pub async fn get_post_list(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }

    get_post_list_db(&app_state.db, query).await.map(|resp| {
        let api_response = ApiResponse::new(
            200,
            "Success".to_string(),
            Some(
                PostListDataBuilder::new()
                    .set_data(resp.0)
                    .set_pagination(resp.1)
                    .build(),
            ),
        );
        HttpResponse::Ok().json(api_response)
    })
}

pub async fn get_trending_posts(
    session: Session,
    app_state: web::Data<AppState>,
    _query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let _ = login_in_unauthentic(&session).await;

    // 获取用户ID或特征，用于推荐
    let user_id = SessionOperation::get_user_id(session).unwrap();

    // 调用深度学习模型进行推荐
    let recommended_post_ids = recommend_posts(app_state.clone(), user_id).await?;

    // 获取推荐的推文
    let posts = get_posts_by_ids(&app_state.db, recommended_post_ids).await?;

    let api_response = ApiResponse::new(
        200,
        "Success".to_string(),
        Some(
            DataBuilder::new()
                .set_data(posts)
                .build(),
        ),
    );

    Ok(HttpResponse::Ok().json(api_response))
}

// Update
/*
curl -X PUT localhost:8000/api/posts/1 \
   -H "Content-Type: application/json" \
   -d '{
       "content": "Modified content."
   }'
*/
pub async fn update_post_details(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
    update_post: web::Json<UpdatePost>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let _ = login_in_unauthentic(&session).await;
    let (post_id, ) = path.into_inner();
    update_post_db(&app_state.db, post_id, update_post.into())
        .await
        .map(|post| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Update Successful".to_string(),
                Some(DataBuilder::new().set_data(post).build()),
            ))
        })
}

// Delete
/*
curl -X DELETE http://localhost:8000/api/posts/1
*/
pub async fn delete_post(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let _ = login_in_unauthentic(&session).await;
    let (post_id, ) = path.into_inner();
    delete_post_db(&app_state.db, post_id).await.map(|post| {
        HttpResponse::Ok().json(ApiResponse::new(
            200,
            "Delete Successful".to_string(),
            Some(DataBuilder::new().set_data(post).build()),
        ))
    })
}

#[cfg(test)]
mod tests {
    use actix_session::SessionExt;
    use actix_web::{http::StatusCode, ResponseError, test, web};
    use serde_json::Value;

    use crate::{
        handlers::post::{
            delete_post, get_post_detail, insert_new_post, insert_post_db, update_post_details,
        },
        models::post::{CreatePost, UpdatePost},
        state::{AppState, get_demo_state},
    };

    #[actix_rt::test]
    async fn test_insert_post() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let new_post_msg = CreatePost::demo();
        let post_param = web::Json(new_post_msg.clone());

        // 发送请求前设置 session 数据
        let session = test::TestRequest::post().to_http_request().get_session();
        session.insert("user_id", 1).unwrap(); // 模拟 user_id 为 1
        let resp = insert_new_post(session, app_state.clone(), post_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // 获取响应 body 中的 JSON 数据
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

        // 从 JSON 中获取 id 字段
        let post_id = body_json["body"]["data"]["id"]
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
        let app_state: web::Data<AppState> = get_demo_state().await;
        // 发送请求前设置 session 数据
        let session = test::TestRequest::post().to_http_request().get_session();
        session.insert("user_id", 1).unwrap(); // 模拟 user_id 为 1
        let post = CreatePost::demo();
        let insert_result = insert_post_db(&app_state.db, post.clone()).await.unwrap();
        assert_eq!(post.content, insert_result.content);
        // Delete test post.
        let delete_params: web::Path<(i32, )> = web::Path::from((insert_result.id, ));
        let resp = delete_post(session, app_state.clone(), delete_params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_post() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let post = CreatePost::demo();
        let insert_result = insert_post_db(&app_state.db, post.clone()).await.unwrap();
        assert_eq!(&post.content, &insert_result.content);
        // Update test user.
        let update_post_msg = UpdatePost {
            content: Some(String::from("test_update_post_after")),
        };
        let parameters: web::Path<(i32, )> = web::Path::from((insert_result.id, ));
        let update_param = web::Json(update_post_msg);
        // 发送请求前设置 session 数据
        let session = test::TestRequest::put().to_http_request().get_session();
        session.insert("user_id", 1).unwrap(); // 模拟 user_id 为 1

        let resp = update_post_details(session, app_state.clone(), parameters, update_param)
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
        let app_state: web::Data<AppState> = get_demo_state().await;
        // 发送请求前设置 session 数据
        let session = test::TestRequest::get().to_http_request().get_session();
        session.insert("user_id", 1).unwrap(); // 模拟 user_id 为 1

        let new_post_msg = CreatePost::demo();
        let result = insert_post_db(&app_state.db, new_post_msg.clone())
            .await
            .unwrap();
        assert_eq!(&new_post_msg.content, &result.content);
        let parameters: web::Path<(i32, )> = web::Path::from((result.id, ));
        let resp = get_post_detail(session, app_state.clone(), parameters).await;
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
