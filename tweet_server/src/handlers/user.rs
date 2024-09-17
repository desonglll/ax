use actix_session::Session;
use actix_web::{HttpResponse, web};

use crate::{
    dbaccess::user::{
        delete_user_db, get_user_detail_db, get_user_list_db, insert_user_db, update_user_db,
    },
    errors::AxError,
    libraries::resp::{api_response::ApiResponse, data::DataBuilder, response::ErrorMsg},
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
pub async fn post_new_user(
    app_state: web::Data<AppState>,
    new_user: web::Json<CreateUser>,
) -> Result<HttpResponse, AxError> {
    insert_user_db(&app_state.db, new_user.into())
        .await
        .map(|user| {
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
pub async fn get_user_detail(
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
) -> Result<HttpResponse, AxError> {
    let (user_id, ) = path.into_inner();
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
pub async fn get_user_list(app_state: web::Data<AppState>) -> Result<HttpResponse, AxError> {
    get_user_list_db(&app_state.db).await.map(|resp| {
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
pub async fn update_user_details(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse, AxError> {
    let (user_id, ) = path.into_inner();
    match session.get::<i32>("user_id") {
        Ok(session_user_id) => {
            if session_user_id.unwrap_or(-1) == user_id {
                update_user_db(&app_state.db, user_id, update_user.into())
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
pub async fn delete_user(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32, )>,
) -> Result<HttpResponse, AxError> {
    let (user_id, ) = path.into_inner();
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
    use std::env;

    use actix_web::{
        http::StatusCode,
        ResponseError,
        web::{self},
    };
    use dotenv::dotenv;
    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{
        dbaccess::user::{check_password_correct_db, insert_user_db},
        handlers::user::{delete_user, get_user_detail, post_new_user, update_user_details},
        models::user::{CreateUser, UpdateUser},
        state::AppState,
    };
    use crate::state::get_demo_state;
    use crate::utils::test::get_test_session;

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
            profile_picture: Some(Uuid::new_v4()),
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
            password: "070011".to_string(),
            full_name: Some("test_full_name".to_owned()),
            phone: Some("12345678900".to_owned()),
            is_active: Some(true),
            is_admin: Some(true),
            profile_picture: Some(Uuid::new_v4()),
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
            profile_picture: Some(Uuid::new_v4()),
        };
        let result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &result.user_name);
        let parameters: web::Path<(i32, )> = web::Path::from((result.id, ));
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
            profile_picture: Some(Uuid::new_v4()),
        };
        let insert_result = insert_user_db(&app_state.db, user.clone()).await.unwrap();
        assert_eq!(&user.user_name, &insert_result.user_name);
        // Delete test user.
        let session = get_test_session(&insert_result).await;
        let delete_params: web::Path<(i32, )> = web::Path::from((insert_result.id, ));
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
            profile_picture: Some(Uuid::new_v4()),
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
        let parameters: web::Path<(i32, )> = web::Path::from((insert_result.id, ));
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
}
