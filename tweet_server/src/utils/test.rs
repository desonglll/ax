use actix_session::{Session, SessionExt};
use actix_web::body::to_bytes;
use actix_web::test::TestRequest;
use actix_web::{FromRequest, HttpResponse};
use serde_json::Value;

use crate::models::user::User;

/// Obtain a user session for testing.
///
/// This function creates a test session and writes the specified user's
/// `is_admin`, `user_name`, and `user_id` into the session.
///
/// # Arguments
///
/// * `USER` - A reference to the user data structure.
///
/// # Returns
///
/// A [`Session`] instance populated with the user details.
pub async fn get_test_session(user: &User) -> Session {
    let req = TestRequest::default().to_http_request();
    let session = Session::from_request(&req, &mut actix_web::dev::Payload::None)
        .await
        .unwrap();
    session.insert("is_admin", user.is_admin).unwrap();
    session.insert("user_name", user.user_name.clone()).unwrap();
    session.insert("user_id", user.id).unwrap();
    session
}

/// Obtain a demo session for testing.
///
/// This function creates a test session, setting `user_id` to 1 and `is_active` to true
/// to simulate a logged-in user.
///
/// # Returns
///
/// A [`Session`] instance representing a logged-in demo user.
pub async fn get_demo_session() -> Session {
    let session = TestRequest::post().to_http_request().get_session();
    session.insert("user_id", 1).unwrap();
    session.insert("is_active", true).unwrap();
    session
}

/// Convert an HTTP response to a JSON value.
///
/// This function reads the body of the HTTP response and parses it into a [`Value`]
/// for assertions in tests.
///
/// # Arguments
///
/// * `RESP` - The HTTP response to be converted.
///
/// # Returns
///
/// The parsed [`Value`] containing the JSON body.
pub async fn http_response_to_json(resp: HttpResponse) -> Value {
    let body = resp.into_body();
    let body_bytes = to_bytes(body).await.unwrap();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    body_json
}
