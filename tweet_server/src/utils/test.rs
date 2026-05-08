use actix_session::{Session, SessionExt};
use actix_web::body::to_bytes;
use actix_web::test::TestRequest;
use actix_web::{FromRequest, HttpResponse};
use serde_json::Value;

use crate::models::user::User;

/// 获取用于测试的用户 session
///
/// 创建一个测试 session，并将指定用户的 `is_admin`、`user_name`、`user_id` 写入 session。
///
/// # 参数
///
/// - `user`: 用户数据引用
///
/// # 返回值
///
/// 返回已设置用户信息的测试 `Session`。
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

/// 获取用于测试的演示 session
///
/// 创建一个测试 session，写入 `user_id=1` 和 `is_active=true`，
/// 模拟已登录用户。
///
/// # 返回值
///
/// 返回已设置演示用户信息的测试 `Session`。
pub async fn get_demo_session() -> Session {
    let session = TestRequest::post().to_http_request().get_session();
    session.insert("user_id", 1).unwrap();
    session.insert("is_active", true).unwrap();
    session
}

/// 将 HttpResponse 转换为 JSON 值
///
/// 读取响应体的字节流并解析为 `serde_json::Value`，用于测试中的响应断言。
///
/// # 参数
///
/// - `resp`: HTTP 响应
///
/// # 返回值
///
/// 返回解析后的 JSON 值。
pub async fn http_response_to_json(resp: HttpResponse) -> Value {
    let body = resp.into_body();
    let body_bytes = to_bytes(body).await.unwrap();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();
    body_json
}
