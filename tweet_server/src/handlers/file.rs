use crate::{
    dbaccess::file::{get_file_details_db, insert_file_db, set_file_deleted_by_checksum_db},
    errors::AxError,
    libraries::log::Log,
    models::file::File,
    state::AppState,
};
use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use sha2::{Digest, Sha256};
use std::fs::File as StdFile;
use std::{
    io::{Read, Seek, SeekFrom, Write},
    sync::atomic::{AtomicUsize, Ordering},
};
use uuid::Uuid;
// =============================================================================
// =============================================================================
// =============================================================================
// 追踪上次打印大小的全局变量
static LAST_LOGGED_SIZE_MB: AtomicUsize = AtomicUsize::new(0);

/// 处理文件上传的异步函数。
///
/// 此函数接收一个会话 `session`，一个数据库连接池 `pool`，以及一个 `Multipart` 类型的表单数据流 `payload`。
/// 它会将上传的文件临时保存，计算文件的哈希值，记录文件的大小，并将文件信息保存到数据库中。
///
/// 函数会逐块处理上传的文件，每当上传大小达到 10MB 时，记录日志信息。
/// 上传完成后，会将临时文件重命名为最终文件名，并将文件信息保存到数据库中。
///
/// # 参数
/// - `session`: 会话对象，用于存储用户的会话信息。
/// - `pool`: 数据库连接池，用于数据库操作。
/// - `payload`: Multipart 表单数据流，用于处理上传的文件数据。
///
/// # 返回值
/// 如果操作成功，返回包含文件信息的 JSON 响应。如果用户未登录，返回 401 未授权的响应。
// pub async fn upload(
//     session: Session,
//     app_state: web::Data<AppState>,
//     mut payload: Multipart,
// ) -> actix_web::Result<impl Responder> {
//     use std::fs::File as StdFile;
//     Log::info(format!("Accessing upload api."));
//     let mut result: Vec<File> = Vec::new();

//     if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
//         let user_name = session.get::<String>("user_name").unwrap().unwrap();
//         Log::info(format!("User {} Logged In.", user_name));

//         while let Some(item) = payload.next().await {
//             let mut field = item?;

//             // 提前获取 Content-Disposition 和文件名，避免重复借用
//             let content_disposition = field.content_disposition().cloned();

//             if let Some(content_disposition) = content_disposition {
//                 if let Some(file_name) = content_disposition.get_filename() {
//                     let file_name = file_name.to_string();
//                     let full_path = get_path(file_name.clone());

//                     let tmp_file_name = format!("{}.tmp", file_name.clone());
//                     let tmp_full_path = get_path(tmp_file_name.clone());
//                     Log::info(format!("tmp_full_path: {}", tmp_full_path));

//                     let mut file = StdFile::create(tmp_full_path.clone())
//                         .map_err(|e| {
//                             eprintln!("Error creating file: {:?}", e);
//                             HttpResponse::InternalServerError().finish()
//                         })
//                         .unwrap();

//                     // Record the file size
//                     let mut size = 0;
//                     let mut hasher = Sha256::new(); // Change to Md5::new() if MD5 is desired

//                     Log::info(format!("Writing File."));
//                     // Write each chunk to the file
//                     while let Some(chunk) = field.next().await {
//                         let chunk = chunk?;

//                         // accumulate file size
//                         size += chunk.len();

//                         // Convert size to megabytes (MB)
//                         let size_mb = size as f64 / (1024.0 * 1024.0);
//                         // Use spawn_blocking to handle synchronous `session.insert`

//                         // Calculate size in MB and log if it has exceeded the threshold
//                         let logged_size_mb = LAST_LOGGED_SIZE_MB.load(Ordering::SeqCst);
//                         if size_mb as usize >= logged_size_mb + 10 {
//                             Log::info(format!("Uploading: {:.2}MB", size_mb));
//                             LAST_LOGGED_SIZE_MB.store(size_mb as usize, Ordering::SeqCst);
//                         }
//                         file.write_all(&chunk)
//                             .map_err(|e| {
//                                 eprintln!("Error writing to file: {:?}", e);
//                                 HttpResponse::InternalServerError().finish()
//                             })
//                             .unwrap();

//                         // Update hash with the chunk
//                         hasher.update(&chunk);
//                     }
//                     Log::info(format!("Writing File Successful."));
//                     Log::info(format!("Changing File Name From *.tmp to real."));

//                     // Rename the temporary file to the final file name
//                     std::fs::rename(tmp_full_path.clone(), full_path.clone())
//                         .map_err(|e| {
//                             eprintln!("Error renaming file: {:?}", e);
//                             HttpResponse::InternalServerError().finish()
//                         })
//                         .unwrap();
//                     Log::info(format!("Changing File Name Successful."));
//                     // -------------------------------------------------------------------

//                     // 产生插入数据库的File对象
//                     let content_type = field.content_type().unwrap().to_string();
//                     // Get the final hash result
//                     let hash_result = format!("{:x}", hasher.finalize());
//                     let hash_hex = hex::encode(hash_result);

//                     let new_file = File::new(
//                         &session,
//                         file_name,
//                         size.try_into().unwrap(),
//                         content_type,
//                         Some(String::from("")),
//                         hash_hex,
//                     );
//                     // 如果数据库中存在path相同的记录，则删掉
//                     match set_file_deleted_by_path_db(&app_state.db, full_path.clone()).await {
//                         Ok(_deleted_record) => Log::info(format!("Delete existed record")),
//                         Err(_) => {}
//                     }
//                     // 插入数据库完成
//                     Log::info(format!("Insert Into File Table."));
//                     // result = FileHandler::handle_upload(&pool, new_file);
//                     result.push(insert_file_db(&app_state.db, new_file).await.unwrap());
//                     Log::info(format!("Insert Into File Table Successful."));
//                 } else if let Some(name) = content_disposition.get_name() {
//                     // 获取字段的值
//                     let mut value = Vec::new();
//                     while let Some(chunk) = field.next().await {
//                         let data = chunk?;
//                         value.extend_from_slice(&data);
//                     }

//                     // 将字段值转换为字符串（假设字段值是文本）
//                     let value_str = String::from_utf8(value)
//                         .unwrap_or_else(|_| "Invalid UTF-8 data".to_string());

//                     println!("Field Name: {}, Value: {}", name, value_str);
//                 }
//             }
//         }
//         Log::info(format!("Operation Finished Successfully"));
//         Ok(HttpResponse::Ok().json(result))
//     } else {
//         Log::info(format!("Please Login To Upload."));
//         Log::info(format!("Operation Finished Unsuccessfully"));
//         Ok(HttpResponse::Ok().json(format!("Please Login To Upload.")))
//     }
// }

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

/// 获取当前工作目录的路径。
///
/// 此函数返回当前运行程序的工作目录的完整路径，以字符串形式返回。
///
/// # 返回值
/// 返回当前工作目录的路径字符串。
///
/// # 示例
/// ```
/// use server::routes::file::get_base_url;
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
/// use server::routes::file::get_path;
/// let path = get_path("example.txt".to_string());
/// println!("File Path: {}", path);
/// ```
pub fn get_path(name: String) -> String {
    let base_url = get_base_url();
    let path = format!("{}/upload/{}", base_url, name);
    path
}

// 编码中文文件名的函数
fn encode_filename(filename: &str) -> String {
    percent_encode(filename.as_bytes(), NON_ALPHANUMERIC).to_string()
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
pub async fn upload(
    session: Session,
    app_state: web::Data<AppState>,
    is_pub: bool,
    mut payload: Multipart,
) -> actix_web::Result<impl Responder> {
    Log::info("Accessing upload API.".to_string());

    if session.get::<bool>("is_login").unwrap().unwrap_or(false) {
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
        Ok(HttpResponse::Ok().json(result))
    } else {
        Log::info("Please log in to upload.".to_string());
        Ok(HttpResponse::Ok().json("Please log in to upload.".to_string()))
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

    match set_file_deleted_by_checksum_db(&app_state.db, new_file.checksum.clone()).await {
        Ok(_) => Log::info("Deleted existing record.".to_string()),
        Err(_) => {}
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
