use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::post::*;
use crate::errors::AxError;
use crate::handlers::auth::login_in_unauthentic;
use crate::extractors::api_response::ApiResponse;
use crate::extractors::data::{DataBuilder, PostListDataBuilder};
use crate::extractors::session::SessionOperation;
use crate::models::post::{CreatePost, UpdatePost};
use crate::services::recommend::recommend_posts;
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
/// Insert a new post record.
///
/// This handler processes request payloads to insert a new post. It retrieves
/// the active user's identifier from the SESSION and delegates the database insert
/// to the persistence layer.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `new_post`: JSON payload representing the post details to create.
///
/// # Returns
///
/// An HTTP response enclosing the created post on success, or an [`AxError`] on failure.
pub async fn insert_new_post(
    session: Session,
    app_state: web::Data<AppState>,
    new_post: web::Json<CreatePost>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let mut new_post: CreatePost = new_post.into();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
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
/// Retrieve details of a post by its identifier.
///
/// This handler queries the database to return the post record matching the target ID.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the post identifier.
///
/// # Returns
///
/// An HTTP response enclosing the post details on success, or an [`AxError`] on failure.
pub async fn get_post_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(uuid::Uuid,)>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    let (post_id,) = path.into_inner();
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
/// Retrieve a list of posts supporting pagination and sorting.
///
/// This handler returns a list of posts filtered and ordered based on optional query arguments.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query parameters containing `order_by`, `sort`, `limit`, and `offset`.
///
/// # Returns
///
/// An HTTP response enclosing the list of posts and pagination metadata, or an [`AxError`] on failure.
pub async fn get_post_list(
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();

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

/// Retrieve recommended/trending posts for the active user.
///
/// This handler leverages a machine learning model to recommend posts based on the active user's
/// feature metrics. It retrieves the user ID from the SESSION, obtains recommended post IDs,
/// and fetches the corresponding post details.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `_query`: Reserved query parameters (currently unused).
///
/// # Returns
///
/// An HTTP response enclosing the list of recommended posts on success, or an [`AxError`] on failure.
pub async fn get_trending_posts(
    session: Session,
    app_state: web::Data<AppState>,
    _query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }

    // Retrieve the user ID or features for recommendation.
    let user_id = SessionOperation::get_user_id(session).unwrap_or(0);

    // Invoke the machine learning recommendation service.
    let recommended_post_ids = recommend_posts(app_state.clone(), user_id).await?;

    // Fetch post records for the recommended identifiers.
    let posts = get_posts_by_ids(&app_state.db, recommended_post_ids).await?;

    let api_response = ApiResponse::new(
        200,
        "Success".to_string(),
        Some(DataBuilder::new().set_data(posts).build()),
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
/// Update details of a post.
///
/// This handler updates a post matching the identifier in the URL path. It validates session
/// activity and ownership before delegating the update to the persistence layer.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the post identifier.
/// - `update_post`: JSON payload representing the fields to modify.
///
/// # Returns
///
/// An HTTP response enclosing the updated post details on success, or an [`AxError`] on failure.
pub async fn update_post_details(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(uuid::Uuid,)>,
    update_post: web::Json<UpdatePost>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let (post_id,) = path.into_inner();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);

    // Perform post ownership check.
    let post = get_post_detail_db(&app_state.db, post_id).await?;
    let is_admin_user = crate::extractors::session::is_admin(session.clone()).await.unwrap_or(false);
    if post.user_id != user_id && !is_admin_user {
        return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::new(
            401,
            "Not authorized to update this post".to_string(),
            None,
        )));
    }

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
/// Delete a post by its identifier.
///
/// This handler deletes a post matching the identifier in the URL path. It validates session
/// activity and ownership before delegating the deletion to the persistence layer.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the post identifier.
///
/// # Returns
///
/// An HTTP response enclosing the deleted post details on success, or an [`AxError`] on failure.
pub async fn delete_post(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(uuid::Uuid,)>,
) -> Result<HttpResponse, AxError> {
    app_state.add_request_count();
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let (post_id,) = path.into_inner();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);

    // Perform post ownership check.
    let post = get_post_detail_db(&app_state.db, post_id).await?;
    let is_admin_user = crate::extractors::session::is_admin(session.clone()).await.unwrap_or(false);
    if post.user_id != user_id && !is_admin_user {
        return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::new(
            401,
            "Not authorized to delete this post".to_string(),
            None,
        )));
    }

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
    use actix_web::{http::StatusCode, test, web, ResponseError};
    use serde_json::Value;

    use crate::{
        handlers::post::{
            delete_post, get_post_detail, get_post_list, insert_new_post, insert_post_db, update_post_details,
            get_trending_posts,
        },
        models::post::{CreatePost, UpdatePost},
        state::{get_demo_state, AppState},
        utils::test::get_demo_session,
    };

    #[actix_rt::test]
    async fn test_insert_post() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let new_post_msg = CreatePost::demo();
        let post_param = web::Json(new_post_msg.clone());

        // Set up session data before sending the request.
        let session = get_demo_session().await;
        let resp = insert_new_post(session, app_state.clone(), post_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        // Retrieve JSON data from the response body.
        let body = resp.into_body();
        let body_bytes = actix_web::body::to_bytes(body).await.unwrap();
        let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

        // Extract the id field from the JSON object.
        let post_id_str = body_json["body"]["data"]["id"]
            .as_str()
            .expect("id not found or not a string");
        let post_id = uuid::Uuid::parse_str(post_id_str).expect("not a valid UUID");

        // Clean up the post inserted for testing.
        sqlx::query!("DELETE FROM posts WHERE id = $1", post_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_post() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        // Set up session data before sending the request.
        let session = get_demo_session().await;
        let post = CreatePost::demo();
        let insert_result = insert_post_db(&app_state.db, post.clone()).await.unwrap();
        assert_eq!(post.content, insert_result.content);
        // Delete test post.
        let delete_params: web::Path<(uuid::Uuid,)> = web::Path::from((insert_result.id,));
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
            title: Some(String::from("test_update_title_after")),
            content: Some(String::from("test_update_post_after")),
        };
        let parameters: web::Path<(uuid::Uuid,)> = web::Path::from((insert_result.id,));
        let update_param = web::Json(update_post_msg);
        // Set up session data before sending the request.
        let session = get_demo_session().await;

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

        let new_post_msg = CreatePost::demo();
        let result = insert_post_db(&app_state.db, new_post_msg.clone())
            .await
            .unwrap();
        assert_eq!(&new_post_msg.content, &result.content);
        let parameters: web::Path<(uuid::Uuid,)> = web::Path::from((result.id,));
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

    #[actix_rt::test]
    async fn test_get_post_list() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let mut query_map = std::collections::HashMap::new();
        query_map.insert("limit".to_string(), "5".to_string());
        query_map.insert("offset".to_string(), "0".to_string());
        let query = Some(web::Query(query_map));
        let resp = get_post_list(app_state, query).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_post_list_invalid_order_by() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let mut query_map = std::collections::HashMap::new();
        query_map.insert("order_by".to_string(), "id; DROP TABLE posts;".to_string());
        let query = Some(web::Query(query_map));
        let resp = get_post_list(app_state, query).await;
        assert!(resp.is_err());
        match resp {
            Err(crate::errors::AxError::InvalidInput(_)) => {}
            _ => panic!("Expected InvalidInput error"),
        }
    }

    #[actix_rt::test]
    async fn test_get_post_list_no_query() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let resp = get_post_list(app_state, None).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_insert_post_unauthorized() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let new_post_msg = CreatePost::demo();
        let post_param = web::Json(new_post_msg);
        let session = test::TestRequest::post().to_http_request().get_session();
        let resp = insert_new_post(session, app_state, post_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = crate::utils::test::http_response_to_json(resp).await;
        assert_eq!(body["code"], 401);
    }

    #[actix_rt::test]
    async fn test_get_trending_posts() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let session = get_demo_session().await;
        let resp = get_trending_posts(session, app_state, None).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body: Value = crate::utils::test::http_response_to_json(resp).await;
        assert_eq!(body["code"], 200);
    }
}
