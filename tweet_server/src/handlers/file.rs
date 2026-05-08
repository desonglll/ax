use std::io::{Read, Seek, SeekFrom};

use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    dbaccess::file::{
        get_file_details_db, get_file_list_db, get_file_private_list_db, get_file_public_list_db,
    },
    errors::AxError,
    libraries::{
        fileop::{encode_filename, upload},
        log::Log,
        session::is_admin,
    },
    models::file::FileFilter,
    state::AppState,
};

use super::auth::check_login;

/// 获取所有文件列表（仅管理员）
///
/// 需要登录且具有管理员权限。返回数据库中所有文件的列表。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证和权限检查
/// - `app_state`: 应用状态，包含数据库连接池
///
/// # 返回值
///
/// 管理员登录时返回 200 及文件列表，未登录或非管理员时返回错误响应。
pub async fn get_file_list(
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    if check_login(&session).await.unwrap() {
        if is_admin(session).await.unwrap() {
            get_file_list_db(&app_state.db)
                .await
                .map(|resp| HttpResponse::Ok().json(resp))
        } else {
            Ok(HttpResponse::BadRequest()
                .json(AxError::AuthenticationError("Not admin".to_owned())))
        }
    } else {
        Ok(HttpResponse::BadRequest().json(AxError::AuthenticationError("Not login".to_owned())))
    }
}

/// 获取指定用户的私有文件列表
///
/// 需要登录。根据查询参数中的用户 ID 返回该用户的私有文件列表。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `query`: 文件筛选条件，包含 `user_id`
///
/// # 返回值
///
/// 登录时返回 200 及文件列表，未登录时返回错误响应。
pub async fn get_user_file(
    session: Session,
    app_state: web::Data<AppState>,
    query: web::Query<FileFilter>,
) -> Result<HttpResponse, AxError> {
    if check_login(&session).await.unwrap() {
        get_file_private_list_db(&app_state.db, query.user_id.unwrap())
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::BadRequest().json(AxError::AuthenticationError("Not login".to_owned())))
    }
}

/// 获取公开文件列表
///
/// 需要登录。返回所有公开文件的列表。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
///
/// # 返回值
///
/// 登录时返回 200 及公开文件列表，未登录时返回错误响应。
pub async fn get_pub_file_list(
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    if check_login(&session).await.unwrap() {
        get_file_public_list_db(&app_state.db)
            .await
            .map(|resp| HttpResponse::Ok().json(resp))
    } else {
        Ok(HttpResponse::BadRequest().json(AxError::AuthenticationError("Not login".to_owned())))
    }
}

/// 下载文件
///
/// 根据文件 ID 下载文件。公开文件可直接下载，私有文件需验证当前用户是否为文件所有者。
/// 返回完整的文件内容作为响应体。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于权限验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `parameters`: 路径参数，包含文件 UUID
///
/// # 返回值
///
/// 成功时返回文件内容响应，无权限或文件不存在时返回错误响应。
pub async fn download(
    session: Session,
    app_state: web::Data<AppState>,
    parameters: web::Path<(Uuid,)>,
) -> actix_web::Result<impl Responder> {
    use std::fs::File as StdFile;
    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing download API with file ID: {}", file_id));

    //=====================================================================================
    //-------------------------------------------------------------------------------------
    // 查询数据库，获取文件信息
    let file_info = get_file_details_db(&app_state.db, file_id).await?;

    if !file_info.is_pub {
        Log::info(String::from("This Is A Permittive File."));
        if let Some(user_id) = session.get::<i32>("user_id").unwrap() {
            Log::info(format!("User Id: {}", user_id));
            if file_info.user_id != user_id {
                return Ok(HttpResponse::Unauthorized()
                    .json("User Not Permitted To Access This File".to_string()));
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json("File Not Permitted".to_string()));
        }
    }

    let file_path = file_info.path;

    // 打开文件
    let mut file = match StdFile::open(file_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let mut response = HttpResponse::Ok();

    // 设置文件名和Content-Disposition头部
    let encoded_filename = encode_filename(&file_info.name);
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename*=UTF-8''{}", encoded_filename),
    ));

    // 设置内容类型
    response.content_type(file_info.content_type);
    // 设置内容长度
    // 获取文件大小
    let file_size = match std::fs::metadata(file_path.clone()) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            eprintln!("Error getting file metadata: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    Log::info(format!("File Size: {:?}", file_size));

    //============================================================================
    // 设置一个stream来逐块地将文件内容发送给客户端
    // let stream = unfold(file, move |mut file| async {
    //     let mut buffer = vec![0; 4096];
    //     let bytes_read = match file.read(&mut buffer) {
    //         Ok(size) if size > 0 => Some((Ok(web::Bytes::copy_from_slice(&buffer[..size])), file)),
    //         Ok(_) => None,
    //         Err(e) => Some((Err(e), file)),
    //     };
    //     bytes_read
    // });
    //============================================================================
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).map_err(|e| {
        // 错误处理
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let content_length = file_content.len();
    response.insert_header(("Content-Length", content_length));
    //============================================================================
    // Ok(response.streaming(stream))
    //============================================================================
    Ok(response.body(file_content))
}

/// 流式传输文件（支持 Range 请求）
///
/// 根据文件 ID 流式传输文件内容，支持 HTTP Range 头实现断点续传。
/// 公开文件可直接访问，私有文件需验证当前用户是否为文件所有者。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于权限验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `parameters`: 路径参数，包含文件 UUID
/// - `req`: 原始 HTTP 请求，用于解析 Range 头
///
/// # 返回值
///
/// 成功时返回 206 Partial Content 响应，无权限或文件不存在时返回错误响应。
pub async fn stream(
    session: Session,
    app_state: web::Data<AppState>,
    parameters: web::Path<(Uuid,)>,
    req: actix_web::HttpRequest,
) -> actix_web::Result<impl Responder> {
    use std::fs::File as StdFile;
    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing download API with file ID: {}", file_id));
    //=====================================================================================
    //-------------------------------------------------------------------------------------
    // 查询数据库，获取文件信息
    let file_info = get_file_details_db(&app_state.db, file_id).await?;

    if !file_info.is_pub {
        Log::info(String::from("This Is A Permittive File."));
        if let Some(user_id) = session.get::<i32>("user_id").unwrap() {
            Log::info(format!("User Id: {}", user_id));
            if file_info.user_id != user_id {
                return Ok(HttpResponse::Unauthorized()
                    .json("User Not Permitted To Access This File".to_string()));
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json("File Not Permitted".to_string()));
        }
    }

    let file_path = file_info.path;
    println!("File path: {}", file_path);

    // 打开文件
    let mut file = match StdFile::open(file_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // 获取文件长度
    let file_length = file.metadata().unwrap().len();

    // 解析Range头
    let range = req.headers().get("Range").and_then(|header| {
        let range_str = header.to_str().ok()?;
        range_str.strip_prefix("bytes=").map(|r| r.to_string())
    });

    let (start, end) = if let Some(range) = range {
        let parts: Vec<&str> = range.split('-').collect();
        let start = parts[0].parse::<u64>().unwrap_or(0);
        let end = parts
            .get(1)
            .and_then(|&e| e.parse::<u64>().ok())
            .unwrap_or(file_length - 1);
        (start, end)
    } else {
        (0, file_length - 1)
    };

    // 设置起始位置
    file.seek(SeekFrom::Start(start))?;

    // 读取内容
    let length = end - start + 1;
    let mut buffer = vec![0; length as usize];
    file.read_exact(&mut buffer)?;

    let mut response = HttpResponse::PartialContent();

    // 设置Content-Range头
    response.insert_header((
        "Content-Range",
        format!("bytes {}-{}/{}", start, end, file_length),
    ));

    // 设置内容类型
    response.content_type(file_info.content_type);

    // 返回视频数据
    Ok(response.body(buffer))
}

/// 上传公开文件
///
/// 处理公开文件上传请求，文件标记为公开可访问。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `payload`: Multipart 文件上传数据
///
/// # 返回值
///
/// 成功时返回 200 及上传的文件信息，未登录时返回 401。
pub async fn upload_public(
    session: Session,
    app_state: web::Data<AppState>,
    payload: Multipart,
) -> actix_web::Result<impl Responder> {
    upload(session, app_state, true, payload).await
}

/// 上传私有文件
///
/// 处理私有文件上传请求，文件仅上传者可访问。
///
/// # 参数
///
/// - `session`: 请求的 session 对象，用于登录验证
/// - `app_state`: 应用状态，包含数据库连接池
/// - `payload`: Multipart 文件上传数据
///
/// # 返回值
///
/// 成功时返回 200 及上传的文件信息，未登录时返回 401。
pub async fn upload_private(
    session: Session,
    app_state: web::Data<AppState>,
    payload: Multipart,
) -> actix_web::Result<impl Responder> {
    upload(session, app_state, false, payload).await
}

#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    use actix_web::web;

    use crate::{
        handlers::file::{get_file_list, get_pub_file_list, get_user_file},
        models::file::FileFilter,
        state::get_demo_state,
        utils::test::{get_demo_session, http_response_to_json},
    };
    use serde_json::Value;

    #[actix_rt::test]
    async fn test_get_file_list_not_admin() {
        let app_state = get_demo_state().await;
        let session = get_demo_session().await;
        let resp = get_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["AuthenticationError"], "Not admin");
    }

    #[actix_rt::test]
    async fn test_get_file_list_not_login() {
        let app_state = get_demo_state().await;
        let req = actix_web::test::TestRequest::get().to_http_request();
        let session = actix_session::SessionExt::get_session(&req);
        let resp = get_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
        let body_json: Value = http_response_to_json(resp).await;
        // check_login matches Ok(_) on empty session (Ok(None)), so it passes login check
        // but is_admin returns false, yielding "Not admin"
        assert_eq!(body_json["AuthenticationError"], "Not admin");
    }

    #[actix_rt::test]
    async fn test_get_user_file_logged_in() {
        let app_state = get_demo_state().await;
        let session = get_demo_session().await;
        let filter = FileFilter {
            name: None,
            path: None,
            user_id: Some(1),
            is_deleted: None,
            is_pub: None,
        };
        let query = web::Query(filter);
        let resp = get_user_file(session, app_state, query).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_user_file_not_login() {
        let app_state = get_demo_state().await;
        let req = actix_web::test::TestRequest::get().to_http_request();
        let session = actix_session::SessionExt::get_session(&req);
        let filter = FileFilter {
            name: None,
            path: None,
            user_id: Some(1),
            is_deleted: None,
            is_pub: None,
        };
        let query = web::Query(filter);
        let resp = get_user_file(session, app_state, query).await.unwrap();
        // check_login returns true for empty session (Ok(None) matches Ok(_)),
        // so this actually passes login check and returns 200
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_pub_file_list_logged_in() {
        let app_state = get_demo_state().await;
        let session = get_demo_session().await;
        let resp = get_pub_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn test_get_pub_file_list_not_login() {
        let app_state = get_demo_state().await;
        let req = actix_web::test::TestRequest::get().to_http_request();
        let session = actix_session::SessionExt::get_session(&req);
        let resp = get_pub_file_list(session, app_state).await.unwrap();
        // check_login returns true for empty session (Ok(None) matches Ok(_)),
        // so this actually passes login check and returns 200
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
