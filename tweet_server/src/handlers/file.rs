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
    infra::log::Log,
    extractors::session::is_admin,
    models::file::FileFilter,
    state::AppState,
};

use super::auth::login_in_unauthentic;
use super::file_ops::{encode_filename, upload};
use crate::extractors::{api_response::ApiResponse, data::DataBuilder};

/// Retrieve the list of all files (Administrator only).
///
/// This handler processes request payloads to retrieve all file metadata from the database.
/// It requires active session status and administrator permissions.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
///
/// # Returns
///
/// An HTTP response enclosing all file records on success, or an authentication error.
pub async fn get_file_list(
    session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    if is_admin(session).await.unwrap_or(false) {
        let resp = get_file_list_db(&app_state.db).await?;
        let api_response = ApiResponse::new(
            200,
            "Query successful".to_string(),
            Some(DataBuilder::new().set_data(resp).build()),
        );
        Ok(HttpResponse::Ok().json(api_response))
    } else {
        Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::new(
            403,
            "Not admin".to_owned(),
            None,
        )))
    }
}

/// Retrieve private files belonging to a specific user.
///
/// This handler processes request payloads to retrieve private file records.
/// It requires active session status.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `query`: URL query filter containing optional `user_id`.
///
/// # Returns
///
/// An HTTP response enclosing private file records on success, or an authentication error.
pub async fn get_user_file(
    session: Session,
    app_state: web::Data<AppState>,
    query: web::Query<FileFilter>,
) -> Result<HttpResponse, AxError> {
    if let Ok(resp) = login_in_unauthentic(&session).await {
        return Ok(resp);
    }
    let user_id = query.user_id.unwrap_or_else(|| {
        session.get::<i32>("user_id").unwrap_or_default().unwrap_or(0)
    });
    let resp = get_file_private_list_db(&app_state.db, user_id).await?;
    let api_response = ApiResponse::new(
        200,
        "Query successful".to_string(),
        Some(DataBuilder::new().set_data(resp).build()),
    );
    Ok(HttpResponse::Ok().json(api_response))
}

/// Retrieve all public files.
///
/// This handler processes request payloads to retrieve public file metadata from the database.
/// It requires active session status.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
///
/// # Returns
///
/// An HTTP response enclosing public file records on success, or an authentication error.
pub async fn get_pub_file_list(
    _session: Session,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, AxError> {
    let resp = get_file_public_list_db(&app_state.db).await?;
    let api_response = ApiResponse::new(
        200,
        "Query successful".to_string(),
        Some(DataBuilder::new().set_data(resp).build()),
    );
    Ok(HttpResponse::Ok().json(api_response))
}

/// Download a file by its identifier.
///
/// This handler returns the file content matching the UUID parameter. Public files
/// are returned directly, whereas private files require validation of user ownership.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `parameters`: Path parameters containing the target file UUID.
///
/// # Returns
///
/// An HTTP response containing the binary content of the file, or an authorization error.
pub async fn download(
    session: Session,
    app_state: web::Data<AppState>,
    parameters: web::Path<(Uuid,)>,
) -> actix_web::Result<impl Responder> {
    use std::fs::File as StdFile;
    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing download API with file ID: {}", file_id));

    // Query the database to retrieve file details.
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

    // Open the file.
    let mut file = match StdFile::open(file_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let mut response = HttpResponse::Ok();

    // Set the Content-Disposition header with encoded filename.
    let encoded_filename = encode_filename(&file_info.name);
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename*=UTF-8''{}", encoded_filename),
    ));

    // Set content type and retrieve file size.
    response.content_type(file_info.content_type);
    let file_size = match std::fs::metadata(file_path.clone()) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            eprintln!("Error getting file metadata: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    Log::info(format!("File Size: {:?}", file_size));

    //============================================================================
    // Set up a stream to send the file content chunk by chunk to the client.
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
        // Handle read errors.
        actix_web::error::ErrorInternalServerError(e)
    })?;

    let content_length = file_content.len();
    response.insert_header(("Content-Length", content_length));
    //============================================================================
    // Ok(response.streaming(stream))
    //============================================================================
    Ok(response.body(file_content))
}

/// Stream a file supporting HTTP Range requests.
///
/// This handler streams file contents matching the UUID parameter, supporting partial range queries
/// for video seeking or resume-download.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `parameters`: Path parameters containing the target file UUID.
/// - `req`: The raw HTTP request to parse the Range header.
///
/// # Returns
///
/// A 206 Partial Content HTTP response enclosing the requested range buffer, or an error.
pub async fn stream(
    session: Session,
    app_state: web::Data<AppState>,
    parameters: web::Path<(Uuid,)>,
    req: actix_web::HttpRequest,
) -> actix_web::Result<impl Responder> {
    use std::fs::File as StdFile;
    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing download API with file ID: {}", file_id));
    // Query the database to retrieve file details.
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

    // Open the file.
    let mut file = match StdFile::open(file_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {:?}", e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // Retrieve file size metadata.
    let file_length = file.metadata().unwrap().len();

    // Parse the Range request header.
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

    // Seek to the requested start position.
    file.seek(SeekFrom::Start(start))?;

    // Read the range data block.
    let length = end - start + 1;
    let mut buffer = vec![0; length as usize];
    file.read_exact(&mut buffer)?;

    let mut response = HttpResponse::PartialContent();

    // Insert the Content-Range response header.
    response.insert_header((
        "Content-Range",
        format!("bytes {}-{}/{}", start, end, file_length),
    ));

    // Set the response Content-Type.
    response.content_type(file_info.content_type);

    // Write the partial content to response.
    Ok(response.body(buffer))
}

/// Upload a public file.
///
/// This handler processes multipart payloads to upload files that are publicly visible.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `payload`: Multipart body containing file data.
///
/// # Returns
///
/// An HTTP response enclosing the uploaded file details on success, or an authentication error.
pub async fn upload_public(
    session: Session,
    app_state: web::Data<AppState>,
    payload: Multipart,
) -> actix_web::Result<impl Responder> {
    upload(session, app_state, true, payload).await
}

/// Upload a private file.
///
/// This handler processes multipart payloads to upload files that are only visible to the owner.
///
/// # Parameters
///
/// - `session`: The session object of the incoming request.
/// - `app_state`: Reference to the shared state of the application.
/// - `payload`: Multipart body containing file data.
///
/// # Returns
///
/// An HTTP response enclosing the uploaded file details on success, or an authentication error.
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
        assert_eq!(resp.status(), StatusCode::FORBIDDEN);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 403);
        assert_eq!(body_json["message"], "Not admin");
    }

    #[actix_rt::test]
    async fn test_get_file_list_not_login() {
        let app_state = get_demo_state().await;
        let req = actix_web::test::TestRequest::get().to_http_request();
        let session = actix_session::SessionExt::get_session(&req);
        let resp = get_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 401);
        assert_eq!(body_json["message"], "Please Login");
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
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 200);
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
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 401);
        assert_eq!(body_json["message"], "Please Login");
    }

    #[actix_rt::test]
    async fn test_get_pub_file_list_logged_in() {
        let app_state = get_demo_state().await;
        let session = get_demo_session().await;
        let resp = get_pub_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 200);
    }

    #[actix_rt::test]
    async fn test_get_pub_file_list_not_login() {
        let app_state = get_demo_state().await;
        let req = actix_web::test::TestRequest::get().to_http_request();
        let session = actix_session::SessionExt::get_session(&req);
        let resp = get_pub_file_list(session, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body_json: Value = http_response_to_json(resp).await;
        assert_eq!(body_json["code"], 200);
    }
}
