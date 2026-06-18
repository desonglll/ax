use std::{
    fs::File as StdFile,
    io::Write,
    sync::atomic::{AtomicUsize, Ordering},
};

use actix_multipart::{Field, Multipart};
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use futures::StreamExt;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    dbaccess::file::{insert_file_db, set_file_deleted_by_checksum_db},
    models::file::File,
    state::AppState,
};

use crate::extractors::{api_response::ApiResponse, data::DataBuilder};
use crate::infra::log::Log;

static LAST_LOGGED_SIZE_MB: AtomicUsize = AtomicUsize::new(0);

/// Process multipart file upload requests.
///
/// This function verifies active session status, parses multipart fields, saves file fields
/// to disk, and records metadata in the database. If a file with a matching checksum exists,
/// it soft-deletes the older record prior to writing.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `is_pub`: Boolean indicating whether the file is public.
/// - `payload`: The multipart data stream.
///
/// # Returns
///
/// An HTTP response enclosing the uploaded file details on success, or an authentication error.
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
        let mut description: Option<String> = None;
        let mut uploaded_files: Vec<(String, String, usize, String, String)> = Vec::new();

        while let Some(item) = payload.next().await {
            let mut field = item?;
            let content_disposition = field.content_disposition().cloned();

            if let Some(content_disposition) = content_disposition {
                if let Some(file_name) = content_disposition.get_filename() {
                    let file_name = file_name.to_string();
                    let temp_id = Uuid::new_v4();
                    let tmp_full_path = get_path(format!("{}.tmp", temp_id));
                    
                    let mut file = StdFile::create(tmp_full_path.clone())
                        .map_err(|e| {
                            eprintln!("Error creating file: {:?}", e);
                            actix_web::error::ErrorInternalServerError(e)
                        })?;
                    
                    let (size, hash_hex) = write_chunks_to_file(&mut field, &mut file).await?;
                    let content_type = field.content_type().unwrap().to_string();
                    
                    uploaded_files.push((file_name, tmp_full_path, size, content_type, hash_hex));
                } else if let Some(name) = content_disposition.get_name() {
                    let value_str = process_text_field(&mut field).await;
                    if name == "description" {
                        description = Some(value_str);
                    }
                }
            }
        }

        for (file_name, tmp_path, size, content_type, checksum) in uploaded_files {
            // Instantiate File model (generates Uuid and final storage path)
            let new_file = File::new(
                &session,
                file_name,
                size as i64,
                content_type,
                description.clone(),
                checksum.clone(),
                is_pub,
            );

            // Rename file to its final UUID-based path
            if let Err(e) = std::fs::rename(tmp_path.clone(), new_file.path.clone()) {
                eprintln!("Error renaming file from temp to final path: {:?}", e);
                let _ = std::fs::remove_file(tmp_path);
                return Ok(HttpResponse::InternalServerError().finish());
            }

            // Soft-delete older duplicate if any
            if let Ok(_) = set_file_deleted_by_checksum_db(&app_state.db, new_file.checksum.clone()).await {
                Log::info("Deleted existing record.".to_string());
            }

            Log::info("Inserting into File table.".to_string());
            match insert_file_db(&app_state.db, new_file).await {
                Ok(saved_file) => {
                    result.push(saved_file);
                }
                Err(e) => {
                    eprintln!("Error inserting file to db: {:?}", e);
                    return Err(e.into());
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

/// Write multipart data chunks to a file.
///
/// This function reads chunks from FIELD, writes them into FILE, and updates
/// a SHA-256 hasher. It logs upload progress at 10MB intervals.
///
/// # Parameters
///
/// - `field`: Reference to the multipart field.
/// - `file`: Reference to the target standard file handle.
///
/// # Returns
///
/// A tuple containing size and hash hex string on success, or an `actix_web::Error`.
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
                actix_web::error::ErrorInternalServerError(e)
            })?;

        hasher.update(&chunk);
    }

    let hash_hex = hex::encode(hasher.finalize());
    Log::info("File writing successful.".to_string());

    Ok((size, hash_hex))
}

/// Read text data from a multipart field.
///
/// This function collects all chunks from FIELD and reconstructs them into a string.
///
/// # Parameters
///
/// - `field`: Reference to the multipart field.
///
/// # Returns
///
/// A string representing the text content of the field.
async fn process_text_field(field: &mut Field) -> String {
    let mut value = Vec::new();
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        value.extend_from_slice(&data);
    }

    String::from_utf8(value).unwrap_or_else(|_| "Invalid UTF-8 data".to_string())
}

/// Retrieve the path of the current working directory.
///
/// This function returns the absolute path of the directory from which the application was started.
///
/// # Returns
///
/// A string representing the base working directory.
///
/// # Examples
/// ```
/// use tweet_server::handlers::file_ops::get_base_url;
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

/// Generate the absolute storage path for a file.
///
/// This function constructs a target path under the uploads folder using the NAME parameter.
///
/// # Parameters
///
/// - `name`: The name of the file.
///
/// # Returns
///
/// The absolute file path string.
///
/// # Examples
/// ```
/// use tweet_server::handlers::file_ops::get_path;
/// let path = get_path("example.txt".to_string());
/// println!("File Path: {}", path);
/// ```
pub fn get_path(name: String) -> String {
    let base_url = get_base_url();
    let path = format!("{}/uploads/{}", base_url, name);
    path
}

// Encode filename using percent-encoding to support non-ASCII characters.
pub fn encode_filename(filename: &str) -> String {
    percent_encode(filename.as_bytes(), NON_ALPHANUMERIC).to_string()
}
