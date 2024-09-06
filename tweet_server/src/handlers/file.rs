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
    let file_size = match std::fs::metadata(&file_path.clone()) {
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
        if range_str.starts_with("bytes=") {
            Some(range_str[6..].to_string())
        } else {
            None
        }
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

pub async fn upload_public(
    session: Session,
    app_state: web::Data<AppState>,
    payload: Multipart,
) -> actix_web::Result<impl Responder> {
    upload(session, app_state, true, payload).await
}

pub async fn upload_private(
    session: Session,
    app_state: web::Data<AppState>,
    payload: Multipart,
) -> actix_web::Result<impl Responder> {
    upload(session, app_state, false, payload).await
}
