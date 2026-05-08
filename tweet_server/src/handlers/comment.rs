use std::collections::HashMap;

use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::dbaccess::comment::{
    delete_comment_by_id_db, get_comment_by_query_db, insert_comment_db,
};
use crate::handlers::auth::login_in_unauthentic;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;
use crate::{errors::AxError, models::comment::CreateComment, state::AppState};

/*
{
    "content": "Test content",
    "reply_to": 1,
    "reactions": "Like",
    "reply_type": "post"
}
 */
/// 插入一条新评论
///
/// 创建评论处理器。从 session 中获取当前用户 ID 并设置到评论数据中，
/// 然后调用数据库层插入评论记录。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证和获取用户 ID
/// - `app_state`: 应用状态，包含数据库连接池
/// - `create_comment`: 请求体中的评论数据
///
/// # 返回值
///
/// 成功时返回 200 响应及创建的评论数据，失败时返回 [`AxError`]。
pub async fn insert_comment(
    session: Session,
    app_state: web::Data<AppState>,
    create_comment: web::Json<CreateComment>,
) -> Result<HttpResponse, AxError> {
    // Login check
    let _ = login_in_unauthentic(&session).await;
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

/// 根据 ID 删除评论
///
/// 删除评论处理器。验证登录状态后，根据路径参数中的评论 ID 删除评论。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `params`: 路径参数，包含评论 ID
///
/// # 返回值
///
/// 成功时返回 200 响应及被删除的评论数据，失败时返回 [`AxError`]。
pub async fn delete_comment(
    session: Session,
    app_state: web::Data<AppState>,
    params: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    // Login check
    let _ = login_in_unauthentic(&session).await;
    let (id,) = params.into_inner();
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

/// 根据查询条件获取评论列表
///
/// 查询评论处理器。验证登录状态后，根据 URL 查询参数（如评论 ID、回复目标等）筛选评论。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: URL 查询参数
///
/// # 返回值
///
/// 成功时返回 200 响应及评论列表，失败时返回 [`AxError`]。
pub async fn get_comment_by_query(
    app_state: web::Data<AppState>,
    query: web::Query<HashMap<String, String>>,
) -> Result<HttpResponse, AxError> {
    get_comment_by_query_db(&app_state.db, query)
        .await
        .map(|comment| {
            let api_response = ApiResponse::new(
                200,
                "Get Comment Successful".to_string(),
                Some(DataBuilder::new().set_data(comment).build()),
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
        let comment_id = body_json["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;
        // 删除测试插入的 post
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
        let comment_id = body_json["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;
        let params = actix_web::web::Path::<(i32,)>::from((comment_id,));
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
        let comment_id = body_json["body"]["data"]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;
        let mut query = HashMap::<String, String>::new();
        query.insert("commentId".to_string(), comment_id.to_string());

        // Test get comment by id.
        let get_resp = get_comment_by_query(session.clone(), app_state.clone(), Query(query))
            .await
            .unwrap();
        let get_body_json: Value = http_response_to_json(get_resp).await;
        println!("{:?}", get_body_json);
        let get_comment_id = get_body_json["body"]["data"][0]["id"]
            .as_i64()
            .expect("id not found or not an integer") as i32;
        assert_eq!(comment_id, get_comment_id);

        // 删除测试插入的 post
        sqlx::query!("DELETE FROM comments WHERE id = $1", comment_id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }
}
