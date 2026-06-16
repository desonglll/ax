use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::reaction::*;
use crate::handlers::auth::login_in_unauthentic;
use crate::extractors::api_response::ApiResponse;
use crate::extractors::data::DataBuilder;
use crate::{errors::AxError, models::reaction::CreateReaction, state::AppState};

// Create
/*
http://localhost:8000/api/reactions/post/like?userId=2&postId=1
*/
/// Insert a like reaction record.
///
/// This handler processes request queries to record a "Like" reaction for a post or comment.
/// It retrieves the active user's identifier from the SESSION and target specifications from QUERY.
/// If a conflicting "Dislike" record exists, it is deleted prior to insertion.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query parameters containing target identifier `toId` and category `toType`.
///
/// # Returns
///
/// An HTTP response enclosing the created reaction details on success, or an [`AxError`] on failure.
pub async fn insert_like_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let to_id = query_map
        .get("toId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let to_type = query_map.get("toType").unwrap_or(&"post".to_string()).clone();

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
/// Insert a dislike reaction record.
///
/// This handler processes request queries to record a "Dislike" reaction for a post or comment.
/// It retrieves the active user's identifier from the SESSION and target specifications from QUERY.
/// If a conflicting "Like" record exists, it is deleted prior to insertion.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query parameters containing target identifier `toId` and category `toType`.
///
/// # Returns
///
/// An HTTP response enclosing the created reaction details on success, or an [`AxError`] on failure.
pub async fn insert_dislike_reaction(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    println!("{:?}", query_map);
    let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
    let to_id = query_map
        .get("toId")
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    let to_type = query_map.get("toType").unwrap_or(&"post".to_string()).clone();
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
/// Retrieve reaction statistics table.
///
/// This handler processes request queries to compute likes and dislikes counts for a target.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query mapping representing target filters containing `toId`.
///
/// # Returns
///
/// An HTTP response enclosing the count metadata on success, or an [`AxError`] on failure.
pub async fn get_single_reaction_table_by_query(
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    let query_wrapper = web::Query(query_map);
    get_reaction_table_by_query_db(&app_state.db, query_wrapper)
        .await
        .map(|reaction_table| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get Reaction Table Successful".to_string(),
                Some(DataBuilder::new().set_data(reaction_table).build()),
            ))
        })
}

/// Retrieve a list of reaction records filtered by query parameters.
///
/// This handler returns a list of reaction records filtered based on query filters.
/// It defaults to the active user's identifier if `userId` is omitted from the QUERY map.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query parameters containing filter arguments.
///
/// # Returns
///
/// An HTTP response enclosing the matched reaction records on success, or an [`AxError`] on failure.
pub async fn get_reactions_by_query(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let mut query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    if query_map.get("userId").is_none() {
        let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(0);
        query_map.insert("userId".to_string(), user_id.to_string());
    }
    let query_wrapper = web::Query(query_map);
    get_reactions_by_query_db(&app_state.db, query_wrapper)
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
/// Delete a reaction record by its identifier.
///
/// This handler processes request queries to delete the reaction record matching `reactionId` from QUERY.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: Optional URL query parameters containing `reactionId`.
///
/// # Returns
///
/// An HTTP response enclosing the deleted reaction record details on success, or an [`AxError`] on failure.
pub async fn delete_reaction_by_id(
    session: Session,
    app_state: web::Data<AppState>,
    query: Option<web::Query<HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    let reaction_id = query_map
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
    use std::collections::HashMap;
    use actix_web::http::StatusCode;
    use actix_web::web;
    use serde_json::Value;

    use crate::{
        handlers::reaction::{
            delete_reaction_by_id, get_single_reaction_table_by_query, insert_dislike_reaction,
            insert_like_reaction,
        },
        state::{get_demo_state, AppState},
        utils::test::{get_demo_session, http_response_to_json},
    };

    #[actix_rt::test]
    async fn test_insert_like_reaction() {
        let app_state: web::Data<AppState> = get_demo_state().await;
        let session = get_demo_session().await;
        let mut query_map = HashMap::new();
        query_map.insert("toId".to_string(), "1".to_string());
        query_map.insert("toType".to_string(), "post".to_string());
        let query = Some(web::Query(query_map));
        
        let resp = insert_like_reaction(session, app_state.clone(), query)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        
        let body: Value = http_response_to_json(resp).await;
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
        let session = get_demo_session().await;
        let mut query_map = HashMap::new();
        query_map.insert("toId".to_string(), "1".to_string());
        query_map.insert("toType".to_string(), "post".to_string());
        let query = Some(web::Query(query_map));

        let resp = insert_dislike_reaction(session, app_state.clone(), query)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body: Value = http_response_to_json(resp).await;
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
        let session = get_demo_session().await;

        // First insert a like
        let mut query_map = HashMap::new();
        query_map.insert("toId".to_string(), "9999".to_string());
        query_map.insert("toType".to_string(), "post".to_string());
        let insert_resp = insert_like_reaction(session, app_state.clone(), Some(web::Query(query_map.clone())))
            .await
            .unwrap();
        assert_eq!(insert_resp.status(), StatusCode::OK);
        let insert_body: Value = http_response_to_json(insert_resp).await;
        let reaction_id = insert_body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found") as i32;

        // Then get the reaction table
        let get_resp = get_single_reaction_table_by_query(app_state.clone(), Some(web::Query(query_map)))
            .await
            .unwrap();
        assert_eq!(get_resp.status(), StatusCode::OK);
        let get_body: Value = http_response_to_json(get_resp).await;
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
        let session = get_demo_session().await;

        // First insert a like
        let mut query_map = HashMap::new();
        query_map.insert("toId".to_string(), "8888".to_string());
        query_map.insert("toType".to_string(), "post".to_string());
        let insert_resp = insert_like_reaction(session.clone(), app_state.clone(), Some(web::Query(query_map)))
            .await
            .unwrap();
        let insert_body: Value = http_response_to_json(insert_resp).await;
        let reaction_id = insert_body["body"]["data"]["id"]
            .as_i64()
            .expect("id not found") as i32;

        // Delete the reaction
        let mut delete_query_map = HashMap::new();
        delete_query_map.insert("reactionId".to_string(), reaction_id.to_string());
        let delete_resp = delete_reaction_by_id(session, app_state.clone(), Some(web::Query(delete_query_map)))
            .await
            .unwrap();
        assert_eq!(delete_resp.status(), StatusCode::OK);
    }
}
