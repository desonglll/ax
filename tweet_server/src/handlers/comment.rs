use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{errors::AxError, models::comment::CreateComment, state::AppState};
use crate::dbaccess::comment::{delete_comment_by_id_db, insert_comment_db};
use crate::handlers::auth::login_in_unauthentic;
use crate::libraries::resp::api_response::ApiResponse;
use crate::libraries::resp::data::DataBuilder;

/*
{
    "content": "Test content",
    "reply_to": 1,
    "reactions": "Like",
    "reply_type": "post"
}
 */
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
    insert_comment_db(&app_state.db, create_comment).await.map(|comment| {
        let api_response = ApiResponse::new(
            200,
            "Create Comment Successful".to_string(),
            Some(DataBuilder::new().set_data(comment).build()),
        );
        HttpResponse::Ok().json(api_response)
    })
}

pub async fn delete_comment(
    session: Session,
    app_state: web::Data<AppState>,
    params: web::Path<(i32, )>,
) -> Result<HttpResponse, AxError> {
    // Login check
    let _ = login_in_unauthentic(&session).await;
    let (id, ) = params.into_inner();
    delete_comment_by_id_db(&app_state.db, id).await.map(|comment| {
        let api_response = ApiResponse::new(
            200,
            "Delete Comment Successful".to_string(),
            Some(DataBuilder::new().set_data(comment).build()),
        );
        HttpResponse::Ok().json(api_response)
    })
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::web::Json;
    use serde_json::Value;

    use crate::handlers::comment::{delete_comment, insert_comment};
    use crate::models::comment::CreateComment;
    use crate::state::get_demo_state;
    use crate::utils::test::{get_demo_session, http_response_to_json};

    #[actix_rt::test]
    async fn test_insert_comment() {
        let new_comment = CreateComment::demo();
        let session = get_demo_session().await;
        let app_state = get_demo_state().await;
        let resp = insert_comment(session, app_state.clone(), Json(new_comment)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id = body_json["body"]["data"]["id"].as_i64().expect("id not found or not an integer") as i32;
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
        let resp = insert_comment(session.clone(), app_state.clone(), Json(new_comment)).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        let comment_id = body_json["body"]["data"]["id"].as_i64().expect("id not found or not an integer") as i32;
        let params = actix_web::web::Path::<(i32, )>::from((comment_id, ));
        let del_resp = delete_comment(session.clone(), app_state.clone(), params).await.unwrap();
        assert_eq!(del_resp.status(), StatusCode::OK);
    }
}