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
/// 创建新用户
///
/// 注册处理器。将请求体中的用户数据插入数据库。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
/// - `new_user`: 请求体中的用户注册数据
///
/// # 返回值
///
/// 成功时返回 200 响应及创建的用户数据，失败时返回 [`AxError`]。
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
/// 根据用户 ID 获取用户详情
///
/// 根据路径参数中的用户 ID 查询用户信息。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
/// - `path`: 路径参数，包含用户 ID
///
/// # 返回值
///
/// 成功时返回 200 响应及用户详情，失败时返回 [`AxError`]。
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

/// 获取当前登录用户的个人资料
///
/// 从 session 中获取当前用户 ID，返回该用户的详细信息。
/// 如果用户未登录，返回 401 提示。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
/// - `session`: 请求的 session 对象，用于获取当前用户 ID
///
/// # 返回值
///
/// 登录时返回 200 响应及用户详情，未登录时返回 401 提示。
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
/// 获取所有用户列表
///
/// 返回数据库中所有用户的列表。
///
/// # 参数
///
/// - `app_state`: 应用状态，包含数据库连接池
///
/// # 返回值
///
/// 成功时返回 200 响应及用户列表，失败时返回 [`AxError`]。
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
/// 更新用户信息
///
/// 验证当前登录用户是否为被修改用户本人后，更新用户信息。
/// 仅更新请求体中提供的字段，未提供的字段保留原值。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于验证用户身份
/// - `app_state`: 应用状态，包含数据库连接池
/// - `path`: 路径参数，包含待更新用户的 ID
/// - `update_user`: 请求体中的更新数据
///
/// # 返回值
///
/// 身份验证通过时返回 200 响应及更新后的用户数据，身份不匹配时返回 401。
pub async fn update_user_details(
    session: Session,
    app_state: web::Data<AppState>,
    path: web::Path<(i32,)>,
    update_user: web::Json<UpdateUser>,
) -> Result<HttpResponse, AxError> {
    let (user_id,) = path.into_inner();
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
/// 删除用户
///
/// 验证当前登录用户是否为被删除用户本人后，从数据库中删除该用户。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于验证用户身份
/// - `app_state`: 应用状态，包含数据库连接池
/// - `path`: 路径参数，包含待删除用户的 ID
///
/// # 返回值
///
/// 身份验证通过时返回 200 响应及被删除的用户数据，身份不匹配时返回 401。
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
    use uuid::Uuid;

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
            profile_picture: Some(Uuid::new_v4()),
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
        let resp = get_user_list(app_state.clone()).await.unwrap();
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
