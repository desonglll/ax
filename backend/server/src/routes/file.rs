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
use futures::StreamExt; // Correct trait import
use query::{entities::file::File, DbPool};
use sha2::{Digest, Sha256};
use shared::{
    lib::{data::Data, log::Log},
    response::api_response::ApiResponse,
};
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};

// 追踪上次打印大小的全局变量
static LAST_LOGGED_SIZE_MB: AtomicUsize = AtomicUsize::new(0);

pub async fn upload(
    session: Session,
    pool: web::Data<DbPool>,
    mut payload: Multipart,
    description: Option<String>,
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
            std::fs::rename(tmp_full_path.clone(), full_path)
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
                description.clone(),
                hash_hex,
            );
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

fn get_base_url() -> String {
    let base_url: String = std::env::current_dir()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    base_url
}

fn get_path(name: String) -> String {
    let base_url = get_base_url();
    let path = format!("{}/upload/{}", base_url, name);
    path
}
