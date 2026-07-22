use actix_session::Session;
use actix_web::{web, HttpResponse};

use crate::{
    dbaccess::user::{
        delete_user_db, get_user_detail_db, get_user_list_db, insert_user_db, update_user_db,
    },
    errors::AxError,
    extractors::{api_response::ApiResponse, data::DataBuilder, response::ErrorMsg},
    models::user::{CreateUser, UpdateUser},
    state::AppState,
};

// Create
/*
curl -X POST localhost:8000/users \
   -H "Content-Type: application/json" \
   -d '{
       "userName": "JohnDoe",
       "email": "johndoe@example.com",
       "password": "password123",
       "fullName": "John Doe",
       "phone": "1234567890",
       "isActive": true,
       "isAdmin": false,
       "profilePicture": null
   }'
*/
/// Create a new user record.
///
/// This handler processes request payloads to register a new user in the database.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `new_user`: JSON payload representing user registration fields.
///
/// # Returns
///
/// An HTTP response enclosing the created user details on success, or an [`AxError`] on failure.
pub async fn post_new_user(
    app_state: web::Data<AppState>,
    new_user: web::Json<CreateUser>,
) -> Result<HttpResponse, AxError> {
    let mut new_user: CreateUser = new_user.into();
    if let Err(msg) = new_user.validate() {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::new(400, msg, None)));
    }
    // Registration is public: privilege flags must never come from the client.
    new_user.is_admin = Some(false);
    // New accounts are active unless explicitly created otherwise; leaving
    // this unset used to insert NULL and violate the NOT NULL constraint.
    new_user.is_active = Some(true);
    new_user.profile_picture = None;
    insert_user_db(&app_state.db, new_user).await.map(|user| {
        HttpResponse::Ok().json(ApiResponse::new(
            200,
            "Create User Success".to_string(),
            Some(DataBuilder::new().set_data(user).build()),
        ))
    })
}

// Read
/*
curl -X GET http://localhost:8000/users/1
*/
/// Retrieve details of a user by their identifier.
///
/// This handler queries the database for user details matching the ID parameter in URL path.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the user identifier.
///
/// # Returns
///
/// An HTTP response enclosing the user details on success, or an [`AxError`] on failure.
pub async fn get_user_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    get_user_detail_db(&app_state.db, user_id)
        .await
        .map(|resp| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get UserDetail Success".to_string(),
                Some(DataBuilder::new().set_data(resp).build()),
            ))
        })
}

/// Retrieve profile details of the active user.
///
/// This handler queries the database using user_id from the SESSION to return active user details.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
/// - `session`: The session object of the incoming request.
///
/// # Returns
///
/// An HTTP response enclosing the active user profile on success, or 401 status if not authenticated.
pub async fn get_user_profile(
    app_state: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, AxError> {
    if let Ok(Some(user_id)) = session.get::<i32>("user_id") {
        println!("{}", user_id);
        get_user_detail_db(&app_state.db, user_id)
            .await
            .map(|user| {
                HttpResponse::Ok().json(ApiResponse::new(
                    200,
                    format!("Get `{}` profile successfully.", user_id),
                    Some(DataBuilder::new().set_data(user).build()),
                ))
            })
    } else {
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::new(
            401,
            String::from("Please Login to store session into redis."),
            None,
        )))
    }
}
/*
curl -X GET http://localhost:8000/users
*/
/// Retrieve the list of all users.
///
/// This handler queries the database to return all user records.
///
/// # Parameters
///
/// - `app_state`: Reference to the shared state of the application.
///
/// # Returns
///
/// An HTTP response enclosing all user records on success, or an [`AxError`] on failure.
pub async fn get_user_list(
    app_state: web::Data<AppState>,
    query: Option<web::Query<std::collections::HashMap<String, String>>>,
) -> Result<HttpResponse, AxError> {
    let query_map = query.map(|q| q.into_inner()).unwrap_or_default();
    let limit = query_map
        .get("limit")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(50);
    let offset = query_map
        .get("offset")
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    get_user_list_db(&app_state.db, limit, offset)
        .await
        .map(|resp| {
            HttpResponse::Ok().json(ApiResponse::new(
                200,
                "Get UserList Success".to_string(),
                Some(DataBuilder::new().set_data(resp).build()),
            ))
        })
}

// Update
/*
curl -X PUT localhost:8000/users/1 \
   -H "Content-Type: application/json" \
   -d '{
       "userName": "JohnHanson"
   }'
*/
/// Update user record details.
///
/// This handler updates fields of the user record matching the identifier in the URL path.
/// It verifies if the user in SESSION matches the target identifier.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the target user identifier.
/// - `update_user`: JSON payload representing fields to modify.
///
/// # Returns
///
/// An HTTP response enclosing updated user details on success, or 401/Unauthorized status on failure.
pub async fn update_user_details(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    let mut update_user: UpdateUser = update_user.into();
    if let Some(name) = update_user.user_name.as_mut() {
        *name = name.trim().to_string();
        if name.len() < 3 || name.len() > 32 {
            return Err(AxError::InvalidInput(
                "userName must be between 3 and 32 characters".to_string(),
            ));
        }
    }
    if let Some(email) = update_user.email.as_mut() {
        *email = email.trim().to_lowercase();
        if !email.contains('@') || email.len() > 254 {
            return Err(AxError::InvalidInput("email is not valid".to_string()));
        }
    }
    if let Some(password) = update_user.password.as_ref() {
        if password.len() < 8 || password.len() > 128 {
            return Err(AxError::InvalidInput(
                "password must be between 8 and 128 characters".to_string(),
            ));
        }
    }
    // Only an admin session may change the is_admin flag; a user editing
    // their own profile cannot self-escalate.
    let is_admin = crate::extractors::session::is_admin(session.clone())
        .await
        .unwrap_or(false);
    if !is_admin {
        update_user.is_admin = None;
        update_user.is_active = None;
    }
    match session.get::<i32>("user_id") {
        Ok(session_user_id) => {
            if session_user_id.unwrap_or(-1) == user_id || is_admin {
                update_user_db(&app_state.db, user_id, update_user)
                    .await
                    .map(|user| {
                        HttpResponse::Ok().json(ApiResponse::new(
                            200,
                            "Update User Success".to_string(),
                            Some(DataBuilder::new().set_data(user).build()),
                        ))
                    })
            } else {
                Ok(HttpResponse::Unauthorized().json(ErrorMsg("Invalid user".to_owned())))
            }
        }
        Err(e) => Err(e.into()),
    }
}

// Delete
/*
curl -X DELETE http://localhost:8000/users/1
 */
/// Delete a user record by its identifier.
///
/// This handler removes the user record matching the identifier in the URL path.
/// It verifies if the user in SESSION matches the target identifier.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `path`: Path parameters containing the target user identifier.
///
/// # Returns
///
/// An HTTP response enclosing deleted user details on success, or 401/Unauthorized status on failure.
pub async fn delete_user(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
    match session.get::<i32>("user_id") {
        Ok(session_user_id) => {
            if session_user_id.unwrap_or(-1) == user_id {
                delete_user_db(&app_state.db, user_id)
                    .await
                    .map(|resp| HttpResponse::Ok().json(resp))
            } else {
                Ok(HttpResponse::Unauthorized().json(ErrorMsg("Invalid user".to_owned())))
            }
        }
        Err(e) => Err(e.into()),
    }
}

#[cfg(test)]
mod user_dbaccess_tests {
    use actix_web::{
        http::StatusCode,
        web::{self},
        ResponseError,
    };

    use crate::state::get_demo_state;
    use crate::utils::test::get_test_session;
    use crate::{
        dbaccess::user::{check_password_correct_db, insert_user_db},
        handlers::user::{
            delete_user, get_user_detail, get_user_list, post_new_user, update_user_details,
        },
        models::user::{CreateUser, UpdateUser},
    };

    #[actix_rt::test]
    async fn test_check_password_correct() {
        let app_state = get_demo_state().await;
        let password = String::from("070011");
        let user = CreateUser {
            user_name: "test_password_correct".to_owned(),
            email: "test_password_correct@gmail.com".to_owned(),
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: None,
        };
        let result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &result.user_name.clone());

        let is_correct =
            check_password_correct_db(&app_state.db, result.user_name.clone(), password.clone())
                .await
                .unwrap();
        assert!(is_correct);

        // Delete test user.
        sqlx::query!(
            "delete from users where user_name = $1",
            result.user_name.clone()
        )
        .execute(&app_state.db)
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn test_insert_user() {
        let app_state = get_demo_state().await;
        let new_user_msg = CreateUser {
            user_name: "test_insert_user".to_owned(),
            email: "test_insert_user@gmail.com".to_owned(),
            password: "07001107001100".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: None,
        };
        let user_param = web::Json(new_user_msg.clone());

        let resp = post_new_user(app_state.clone(), user_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        // Delete test user.
        sqlx::query!(
            "delete from users where user_name = $1",
            new_user_msg.user_name
        )
        .execute(&app_state.db)
        .await
        .unwrap();
    }

    #[actix_rt::test]
    async fn test_get_user_detail() {
        let app_state = get_demo_state().await;
        let user = CreateUser {
            user_name: "test_get_user_detail".to_owned(),
            email: "test_get_user_detail@gmail.com".to_owned(),
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: None,
        };
        let result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &result.user_name);
        let parameters: web::Path<(i32,)> = web::Path::from((result.id,));
        let resp = get_user_detail(app_state.clone(), parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }

        // Delete test user.
        sqlx::query!("delete from users where user_name = $1", result.user_name)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_delete_user() {
        let app_state = get_demo_state().await;
        let user = CreateUser {
            user_name: "test_delete_user".to_owned(),
            email: "test_delete_user@gmail.com".to_owned(),
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: None,
        };
        let insert_result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &insert_result.user_name);
        // Delete test user.
        let session = get_test_session(&insert_result).await;
        let delete_params: web::Path<(i32,)> = web::Path::from((insert_result.id,));
        let resp = delete_user(session, app_state.clone(), delete_params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_update_user() {
        let app_state = get_demo_state().await;
        let user = CreateUser {
            user_name: "test_update_user".to_owned(),
            email: "test_update_user@gmail.com".to_owned(),
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: None,
        };
        let insert_result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &insert_result.user_name);
        // Update test user.
        let update_user_msg = UpdateUser {
            user_name: Some("updated_user_name".to_owned()),
            password: None,
            email: None,
            full_name: None,
            phone: None,
            is_active: None,
            is_admin: None,
            profile_picture: None,
        };
        let parameters: web::Path<(i32,)> = web::Path::from((insert_result.id,));
        let update_param = web::Json(update_user_msg);
        let session = get_test_session(&insert_result).await;
        let resp = update_user_details(session, app_state.clone(), parameters, update_param)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        // Delete test user.
        sqlx::query!("delete from users where id = $1", insert_result.id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_get_user_list() {
        let app_state = get_demo_state().await;
        let resp = get_user_list(app_state.clone(), None).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_user_profile_logged_in() {
        use crate::handlers::user::get_user_profile;
        let app_state = get_demo_state().await;
        // Insert a test user first
        let user = CreateUser {
            user_name: "test_profile_user".to_owned(),
            email: "test_profile_user@gmail.com".to_owned(),
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(false),
            profile_picture: None,
        };
        let insert_result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        let session = get_test_session(&insert_result).await;
        let resp = get_user_profile(app_state.clone(), session).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        // Delete test user
        sqlx::query!("delete from users where id = $1", insert_result.id)
            .execute(&app_state.db)
            .await
            .unwrap();
    }

    #[actix_rt::test]
    async fn test_get_user_profile_not_logged_in() {
        use crate::handlers::user::get_user_profile;
        let app_state = get_demo_state().await;
        let session = actix_session::SessionExt::get_session(
            &actix_web::test::TestRequest::get().to_http_request(),
        );
        let resp = get_user_profile(app_state.clone(), session).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: serde_json::Value = crate::utils::test::http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 401);
    }
}
