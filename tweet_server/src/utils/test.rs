use actix_session::{Session, SessionExt};
use actix_web::{FromRequest, HttpResponse};
use actix_web::body::to_bytes;
use actix_web::test::TestRequest;
use serde_json::Value;

use crate::models::user::User;

pub async fn get_test_session(user: &User) -> Session {
    // 使用 TestRequest 模拟带有 session 的 HTTP 请求
    let req = TestRequest::default().to_http_request();
    let session = Session::from_request(&req, &mut actix_web::dev::Payload::None)
        .await
        .unwrap();
    session.insert("is_admin", user.is_admin).unwrap();
    session.insert("user_name", user.user_name.clone()).unwrap();
    session.insert("user_id", user.id).unwrap();
    session
}

pub async fn get_demo_session() -> Session {
    // 发送请求前设置 session 数据
    let session = TestRequest::post()
        .to_http_request()
        .get_session();
    session.insert("user_id", 1).unwrap(); // 模拟 user_id 为 1
    session.insert("is_active", true).unwrap(); // 模拟 user_id 为 1
    session
}

pub async fn http_response_to_json(resp: HttpResponse) -> Value {
    let body = resp.into_body();
    let body_bytes = to_bytes(body).await.unwrap();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    body_json
}