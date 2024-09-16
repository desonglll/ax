use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{errors::AxError, models::comment::CreateComment, state::AppState};
use crate::dbaccess::comment::insert_comment_db;
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
            "Create Comment Success".to_string(),
            Some(DataBuilder::new().set_data(comment).build()),
        );
        HttpResponse::Ok().json(api_response)
    })
}
