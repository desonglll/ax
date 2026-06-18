use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::comment::{
    delete_comment_by_id_db, get_comment_by_query_db, insert_comment_db,
};
use crate::handlers::auth::login_in_unauthentic;
use crate::extractors::api_response::ApiResponse;
use crate::extractors::data::DataBuilder;
use crate::{errors::AxError, models::comment::CreateComment, state::AppState};

/*
{
    "content": "Test content",
    "reply_to": 1,
    "reactions": "Like",
    "reply_type": "post"
}
 */
/// Insert a new comment record.
///
/// This handler processes request payloads to insert a new comment. It retrieves
/// the active user's identifier from the SESSION and delegates the database insert
/// to the persistence layer.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `create_comment`: JSON payload representing the comment details.
///
/// # Returns
///
/// An HTTP response enclosing the created comment on success, or an [`AxError`] on failure.
pub async fn insert_comment(
    session: Session,
    app_state: web::Data<AppState>,
    create_comment: web::Json<CreateComment>,
) -> Result<HttpResponse, AxError> {
    // Perform authentication check.
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id: i32 = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let mut create_comment = create_comment.into_inner();
    create_comment.set_user_id(Some(user_id));
    insert_comment_db(&app_state.db, create_comment)
        .await
        .map(|comment| {
            let api_response = ApiResponse::new(
                200,
                "Create Comment Successful".to_string(),
                Some(DataBuilder::new().set_data(comment).build()),
            );
            HttpResponse::Ok().json(api_response)
        })
}

/// Delete a comment record by its identifier.
///
/// This handler processes request payloads to delete a comment. It verifies the login
/// status, checks ownership or administrator privileges, and performs the deletion.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `params`: Path parameters containing the comment identifier.
///
/// # Returns
///
/// An HTTP response enclosing the deleted comment on success, or an [`AxError`] on failure.
pub async fn delete_comment(
    session: Session,
    app_state: web::Data<AppState>,
    params: web::Path<(uuid::Uuid,)>,
) -> Result<HttpResponse, AxError> {
    // Perform authentication check.
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let (id,) = params.into_inner();

    // Verify commenter ownership.
    let comment = sqlx::query!("select user_id from comments where id = $1", id)
        .fetch_one(&app_state.db)
        .await
        .map_err(|_| AxError::NotFound("Comment not found".to_string()))?;
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let is_admin_user = crate::extractors::session::is_admin(session.clone()).await.unwrap_or(false);
    if comment.user_id != user_id && !is_admin_user {
        return Ok(HttpResponse::Unauthorized().json(ApiResponse::<()>::new(
            401,
            "Not authorized to delete this comment".to_string(),
            None,
        )));
    }

    delete_comment_by_id_db(&app_state.db, id)
        .await
        .map(|comment| {
            let api_response = ApiResponse::new(
                200,
                "Delete Comment Successful".to_string(),
                Some(DataBuilder::new().set_data(comment).build()),
            );
            HttpResponse::Ok().json(api_response)
        })
}

/// Retrieve a list of comments matching the query.
///
/// This handler returns a list of comment records filtered by query parameters.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: URL query mapping representing comment filters.
///
/// # Returns
///
/// An HTTP response enclosing matching comment records on success, or an [`AxError`] on failure.
pub async fn get_comment_by_query(
    app_state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, AxError> {
    get_comment_by_query_db(&app_state.db, query)
        .await
        .map(|(comment, pagination)| {
            let api_response = ApiResponse::new(
                200,
                "Get Comment Successful".to_string(),
                Some(
                    DataBuilder::new()
                        .set_data(comment)
                        .set_pagination(pagination)
                        .build(),
                ),
            );
            HttpResponse::Ok().json(api_response)
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use actix_web::http::StatusCode;
    use actix_web::web::{Json, Query};
    use serde_json::Value;

    use crate::handlers::comment::{delete_comment, get_comment_by_query, insert_comment};
    use crate::models::comment::CreateComment;
    use crate::state::get_demo_state;
    use crate::utils::test::{get_demo_session, http_response_to_json};

    #[actix_rt::test]
    async fn test_insert_comment() {
        let new_comment = CreateComment::demo();
        let session = get_demo_session().await;
        let app_state = get_demo_state().await;
        let resp = insert_comment(session, app_state.clone(), Json(new_comment))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id_str = body_json["body"]["data"]["id"]
            .as_str()
            .expect("id not found or not a string");
        let comment_id = uuid::Uuid::parse_str(comment_id_str).expect("not a valid UUID");
        // Clean up the comment inserted for testing.
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_comment() {
        let new_comment = CreateComment::demo();
        let session = get_demo_session().await;
        let app_state = get_demo_state().await;
        let resp = insert_comment(session.clone(), app_state.clone(), Json(new_comment))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id_str = body_json["body"]["data"]["id"]
            .as_str()
            .expect("id not found or not a string");
        let comment_id = uuid::Uuid::parse_str(comment_id_str).expect("not a valid UUID");
        let params = actix_web::web::Path::<(uuid::Uuid,)>::from((comment_id,));
        let del_resp = delete_comment(session.clone(), app_state.clone(), params)
            .await
            .unwrap();
        assert_eq!(del_resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_comment() {
        let new_comment = CreateComment::demo();
        let session = get_demo_session().await;
        let app_state = get_demo_state().await;
        let resp = insert_comment(session.clone(), app_state.clone(), Json(new_comment))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id_str = body_json["body"]["data"]["id"]
            .as_str()
            .expect("id not found or not a string");
        let comment_id = uuid::Uuid::parse_str(comment_id_str).expect("not a valid UUID");
        let mut query = HashMap::<String, String>::new();
        query.insert("commentId".to_string(), comment_id.to_string());

        // Test get comment by id.
        let get_resp = get_comment_by_query(app_state.clone(), Query(query))
            .await
            .unwrap();
        let get_body_json: Value = http_response_to_json(get_resp).await;
        println!("{:?}", get_body_json);
        let get_comment_id_str = get_body_json["body"]["data"][0]["id"]
            .as_str()
            .expect("id not found or not a string");
        let get_comment_id = uuid::Uuid::parse_str(get_comment_id_str).expect("not a valid UUID");
        assert_eq!(comment_id, get_comment_id);

        // Clean up the comment inserted for testing.
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_get_comment_pagination() {
        let new_comment = CreateComment::demo();
        let session = get_demo_session().await;
        let app_state = get_demo_state().await;
        let resp = insert_comment(session.clone(), app_state.clone(), Json(new_comment))
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id_str = body_json["body"]["data"]["id"]
            .as_str()
            .expect("id not found or not a string");
        let comment_id = uuid::Uuid::parse_str(comment_id_str).expect("not a valid UUID");

        // Query with pagination limit=1
        let mut query = HashMap::<String, String>::new();
        query.insert("commentId".to_string(), comment_id.to_string());
        query.insert("limit".to_string(), "1".to_string());
        query.insert("offset".to_string(), "0".to_string());

        let get_resp = get_comment_by_query(app_state.clone(), Query(query))
            .await
            .unwrap();
        let get_body_json: Value = http_response_to_json(get_resp).await;
        let comments = get_body_json["body"]["data"].as_array().unwrap();
        assert_eq!(comments.len(), 1);
        assert_eq!(get_body_json["body"]["pagination"]["limit"], 1);
        assert_eq!(get_body_json["body"]["pagination"]["offset"], 0);

        // Cleanup
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
