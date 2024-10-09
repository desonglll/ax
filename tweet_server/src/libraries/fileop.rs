use std::{
    fs::File as StdFile,
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{HttpResponse, Responder, web};
use futures::StreamExt;
use percent_encoding::{NON_ALPHANUMERIC, percent_encode};
use sha2::{Digest, Sha256};

use crate::{
    dbaccess::file::{insert_file_db, set_file_deleted_by_checksum_db},
    errors::AxError,
    models::file::File,
    state::AppState,
};

use super::{
    log::Log,
    resp::{api_response::ApiResponse, data::DataBuilder},
};

static LAST_LOGGED_SIZE_MB: AtomicUsize = AtomicUsize::new(0);

pub async fn upload(
    session: Session,
    app_state: web::Data<AppState>,
    is_pub: bool,
    mut payload: Multipart,
) -> actix_web::Result<impl Responder> {
    Log::info("Accessing upload API.".to_string());

    if session.get::<bool>("is_active").unwrap().unwrap_or(false) {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User {} logged in.", user_name));

        let mut result: Vec<File> = Vec::new();

        while let Some(item) = payload.next().await {
            let mut field = item?;
            let content_disposition = field.content_disposition().cloned();

            if let Some(content_disposition) = content_disposition {
                if let Some(file_name) = content_disposition.get_filename() {
                    let file_result =
                        process_file_field(&mut field, file_name, is_pub, &session, &app_state)
                            .await;
                    result.push(file_result.unwrap());
                } else if let Some(name) = content_disposition.get_name() {
                    let value_str = process_text_field(&mut field).await;
                    println!("Field Name: {}, Value: {}", name, value_str);
                }
            }
        }

        Log::info("Operation finished successfully.".to_string());
        let api_response = ApiResponse::new(
            200,
            "Uploaded".to_string(),
            Some(DataBuilder::new().set_data(result).build()),
        );
        Ok(HttpResponse::Ok().json(api_response))
    } else {
        Log::info("Please log in to upload.".to_string());
        Ok(HttpResponse::Ok().json(ApiResponse::<()>::new(
            401,
            "Please log in to upload.".to_string(),
            None,
        )))
    }
}

async fn process_file_field(
    field: &mut Field,
    file_name: &str,
    is_pub: bool,
    session: &Session,
    app_state: &web::Data<AppState>,
) -> Result<File, AxError> {
    let file_name = file_name.to_string();
    let full_path = get_path(file_name.clone());
    let tmp_full_path = get_path(format!("{}.tmp", file_name.clone()));

    Log::info(format!("tmp_full_path: {}", tmp_full_path));

    let mut file = StdFile::create(tmp_full_path.clone())
        .map_err(|e| {
            eprintln!("Error creating file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let (size, hash_hex) = write_chunks_to_file(field, &mut file).await?;
    rename_file(tmp_full_path, full_path.clone())?;

    let content_type = field.content_type().unwrap().to_string();
    let new_file = File::new(
        session,
        file_name,
        size.try_into().unwrap(),
        content_type,
        Some(String::from("")),
        hash_hex,
        is_pub,
    );

    if (set_file_deleted_by_checksum_db(&app_state.db, new_file.checksum.clone()).await).is_ok() {
        Log::info("Deleted existing record.".to_string())
    }

    Log::info("Inserting into File table.".to_string());
    insert_file_db(&app_state.db, new_file).await
}

async fn write_chunks_to_file(
    field: &mut Field,
    file: &mut StdFile,
) -> Result<(usize, String), actix_web::Error> {
    let mut size = 0;
    let mut hasher = Sha256::new();

    Log::info("Writing file.".to_string());
    while let Some(chunk) = field.next().await {
        let chunk = chunk?;
        size += chunk.len();
        let size_mb = size as f64 / (1024.0 * 1024.0);

        if size_mb as usize >= LAST_LOGGED_SIZE_MB.load(Ordering::SeqCst) + 10 {
            Log::info(format!("Uploading: {:.2}MB", size_mb));
            LAST_LOGGED_SIZE_MB.store(size_mb as usize, Ordering::SeqCst);
        }

        file.write_all(&chunk)
            .map_err(|e| {
                eprintln!("Error writing to file: {:?}", e);
                HttpResponse::InternalServerError().finish()
            })
            .unwrap();

        hasher.update(&chunk);
    }

    let hash_hex = hex::encode(format!("{:x}", hasher.finalize()));
    Log::info("File writing successful.".to_string());

    Ok((size, hash_hex))
}

fn rename_file(old_path: String, new_path: String) -> Result<(), actix_web::Error> {
    std::fs::rename(old_path.clone(), new_path.clone())
        .map_err(|e| {
            eprintln!("Error renaming file: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();
    Log::info("File renamed successfully.".to_string());
    Ok(())
}

async fn process_text_field(field: &mut Field) -> String {
    let mut value = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        value.extend_from_slice(&data);
    }

    String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8 data".to_string())
}

/// 获取当前工作目录的路径。
///
/// 此函数返回当前运行程序的工作目录的完整路径，以字符串形式返回。
///
/// # 返回值
/// 返回当前工作目录的路径字符串。
///
/// # 示例
/// ```
/// use tweet_server::libraries::fileop::get_base_url;
/// let base_url = get_base_url();
/// println!("Base URL: {}", base_url);
/// ```
pub fn get_base_url() -> String {
    let base_url: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    base_url
}

/// 根据文件名生成完整的文件保存路径。
///
/// 此函数使用当前工作目录路径和提供的文件名生成完整的文件保存路径。
///
/// # 参数
/// - `name`: 文件名，以字符串形式传入。
///
/// # 返回值
/// 返回完整的文件保存路径。
///
/// # 示例
/// ```
/// use tweet_server::libraries::fileop::get_path;
/// let path = get_path("example.txt".to_string());
/// println!("File Path: {}", path);
/// ```
pub fn get_path(name: String) -> String {
    let base_url = get_base_url();
    let path = format!("{}/uploads/{}", base_url, name);
    path
}

// 编码中文文件名的函数
pub fn encode_filename(filename: &str) -> String {
    percent_encode(filename.as_bytes(), NON_ALPHANUMERIC).to_string()
}
