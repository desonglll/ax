// use std::fs::OpenOptions;
// use std::io::Write;

// use actix_multipart::form::MultipartForm;
// use actix_session::Session;
// use actix_web::{web, HttpResponse, Responder};

// use query::entities::file::UploadForm;
// use query::DbPool;
// use shared::lib::log::Log;

// use crate::handlers::file::FileHandler;
// pub fn upload(
//     session: Session,
//     pool: web::Data<DbPool>,
//     MultipartForm(form): MultipartForm<UploadForm>,
// ) -> impl Responder {
//     Log::info("Accessing upload route".to_string());
//     if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
//         let _user_name = session.get::<String>("user_name").unwrap().unwrap();
//         let result = FileHandler::handle_upload(&session, pool, form);
//         HttpResponse::Ok().json(result)
//     } else {
//         HttpResponse::Unauthorized().body("Please log in.")
//     }
// }

use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder, Result};
use futures::stream::unfold;
use futures::StreamExt;
use sha2::{Digest, Sha256};
use shared::response::api_response::StatusCode;
use std::io::{Read, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use uuid::Uuid;

// Correct trait import
use query::{entities::file::File, DbPool};
use shared::{
    lib::{data::Data, log::Log},
    response::api_response::ApiResponse,
};

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
pub async fn upload(
    session: Session,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
) -> Result<impl Responder> {
    use std::fs::File as StdFile;
    Log::info(format!("Accessing upload api."));
    let mut result: Data<Vec<File>> = Data::default();

    if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
        let user_name = session.get::<String>("user_name").unwrap().unwrap();
        Log::info(format!("User {} Logged In.", user_name));

        // Iterate over multipart stream
        while let Some(item) = payload.next().await {
            let mut field = item?;
            // Create or open a file to write the uploaded data to
            let file_name = field
                .content_disposition()
                .unwrap()
                .get_filename()
                .unwrap()
                .to_string();
            let full_path = get_path(file_name.clone());

            let tmp_file_name = format!("{}.tmp", file_name.clone());
            let tmp_full_path = get_path(tmp_file_name.clone());
            Log::info(format!("tmp_full_path: {}", tmp_full_path));

            let mut file = StdFile::create(tmp_full_path.clone())
                .map_err(|e| {
                    eprintln!("Error creating file: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                })
                .unwrap();

            // Record the file size
            let mut size = 0;
            let mut hasher = Sha256::new(); // Change to Md5::new() if MD5 is desired

            Log::info(format!("Writing File."));
            // Write each chunk to the file
            while let Some(chunk) = field.next().await {
                let chunk = chunk?;

                // accumulate file size
                size += chunk.len();

                // Convert size to megabytes (MB)
                let size_mb = size as f64 / (1024.0 * 1024.0);

                // Calculate size in MB and log if it has exceeded the threshold
                let logged_size_mb = LAST_LOGGED_SIZE_MB.load(Ordering::SeqCst);
                if size_mb as usize >= logged_size_mb + 10 {
                    Log::info(format!("Uploading: {:.2}MB", size_mb));
                    LAST_LOGGED_SIZE_MB.store(size_mb as usize, Ordering::SeqCst);
                }
                file.write_all(&chunk)
                    .map_err(|e| {
                        eprintln!("Error writing to file: {:?}", e);
                        HttpResponse::InternalServerError().finish()
                    })
                    .unwrap();

                // Update hash with the chunk
                hasher.update(&chunk);
            }
            Log::info(format!("Writing File Successful."));
            Log::info(format!("Changing File Name From *.tmp to real."));

            // Rename the temporary file to the final file name
            std::fs::rename(tmp_full_path.clone(), full_path.clone())
                .map_err(|e| {
                    eprintln!("Error renaming file: {:?}", e);
                    HttpResponse::InternalServerError().finish()
                })
                .unwrap();
            Log::info(format!("Changing File Name Successful."));
            // -------------------------------------------------------------------

            // 产生插入数据库的File对象
            // Get file name from content disposition
            let content_disposition = field.content_disposition().unwrap();
            let name = content_disposition.get_filename().unwrap().to_string();
            let content_type = field.content_type().unwrap().to_string();
            // Get the final hash result
            let hash_result = format!("{:x}", hasher.finalize());
            let hash_hex = hex::encode(hash_result);

            let new_file = File::new(
                &session,
                name,
                size.try_into().unwrap(),
                content_type,
                Some(String::from("")),
                hash_hex,
            );
            // 如果数据库中存在path相同的记录，则删掉
            match File::delete_file_by_path(&pool, full_path.clone()) {
                Ok(deleted_record) => {
                    Log::info(format!("Delete existed record: {}", deleted_record.data.id))
                }
                Err(_) => {}
            }
            // 插入数据库完成
            Log::info(format!("Insert Into File Table."));
            // result = FileHandler::handle_upload(&pool, new_file);
            result.data.push(new_file.insert_file(&pool).unwrap().data);
            Log::info(format!("Insert Into File Table Successful."));
        }
        Log::info(format!("Operation Finished Successfully"));
        Ok(HttpResponse::Ok().json(ApiResponse::success(
            "Upload Successful".to_string(),
            Some(result),
        )))
    } else {
        Log::info(format!("Please Login To Upload."));
        Log::info(format!("Operation Finished Unsuccessfully"));
        Ok(HttpResponse::Unauthorized().body("Please log in."))
    }
}
pub async fn download(
    session: Session,
    pool: web::Data<DbPool>,
    file_id: web::Path<Uuid>,
) -> Result<impl Responder> {
    use std::fs::File as StdFile;
    Log::info(format!("Accessing download API with file ID: {}", file_id));

    // if let Some(_is_login) = session.get::<bool>("is_login").unwrap() {
    //     let _user_name = session.get::<String>("user_name").unwrap().unwrap();

    //-------------------------------------------------------------------------------------
    // 查询数据库，获取文件信息
    let file_info = File::get_file(&pool, *file_id).unwrap();

    if file_info.data.user_id != -1 {
        Log::info(String::from("This Is A Permittive File."));
        if let Some(user_id) = session.get::<i32>("user_id").unwrap() {
            Log::info(format!("User Id: {}", user_id));
            if file_info.data.user_id != user_id {
                return Ok(HttpResponse::Unauthorized().json(ApiResponse::<File>::new(
                    StatusCode::Unauthorized,
                    "User Not Permitted To Access This File".to_string(),
                    None,
                )));
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json(ApiResponse::<File>::new(
                StatusCode::Unauthorized,
                "File Not Permitted".to_string(),
                None,
            )));
        }
    }

    let file_path = file_info.data.path;

    // 打开文件
    let file = match StdFile::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let mut response = HttpResponse::Ok();

    // 设置文件名和Content-Disposition头部
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename=\"{}\"", file_info.data.name),
    ));

    // 设置内容类型
    response.content_type(file_info.data.content_type);

    // 设置一个stream来逐块地将文件内容发送给客户端
    let stream = unfold(file, move |mut file| async {
        let mut buffer = vec![0; 4096];
        let bytes_read = match file.read(&mut buffer) {
            Ok(size) if size > 0 => Some((Ok(web::Bytes::copy_from_slice(&buffer[..size])), file)),
            Ok(_) => None,
            Err(e) => Some((Err(e), file)),
        };
        bytes_read
    });
    //-------------------------------------------------------------------------------------

    Ok(response.streaming(stream))
    // } else {
    //     Ok(HttpResponse::Unauthorized().body("Please log in."))
    // }
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
/// let base_url = get_base_url();
/// println!("Base URL: {}", base_url);
/// ```
fn get_base_url() -> String {
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
/// let path = get_path("example.txt".to_string());
/// println!("File Path: {}", path);
/// ```
fn get_path(name: String) -> String {
    let base_url = get_base_url();
    let path = format!("{}/upload/{}", base_url, name);
    path
}
