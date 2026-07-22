use actix_multipart::Multipart;
use actix_session::Session;
use actix_web::{web, HttpResponse, Responder};
use uuid::Uuid;

use crate::{
    dbaccess::file::{
        get_file_details_db, get_file_list_db, get_file_private_list_db, get_file_public_list_db,
    },
    errors::AxError,
    extractors::session::is_admin,
    infra::log::Log,
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
    let session_user_id = session
        .get::<i32>("user_id")
        .unwrap_or_default()
        .unwrap_or(0);
    // Private listings are restricted to the session owner; only an admin may
    // inspect another user's files via the `user_id` query parameter.
    let user_id = match query.user_id {
        Some(requested) if requested != session_user_id => {
            if is_admin(session.clone()).await.unwrap_or(false) {
                requested
            } else {
                return Ok(HttpResponse::Forbidden().json(ApiResponse::<()>::new(
                    403,
                    "Cannot list another user's private files".to_owned(),
                    None,
                )));
            }
        }
        _ => session_user_id,
    };
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
    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing download API with file ID: {}", file_id));

    // Query the database to retrieve file details.
    let file_info = get_file_details_db(&app_state.db, file_id).await?;

    if !file_info.is_pub {
        if let Some(user_id) = session.get::<i32>("user_id").ok().flatten() {
            if file_info.user_id != user_id {
                return Ok(HttpResponse::Unauthorized()
                    .json("User Not Permitted To Access This File".to_string()));
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json("File Not Permitted".to_string()));
        }
    }

    // Open asynchronously and stream in chunks: no blocking the executor and
    // no buffering the whole file in memory.
    let file = match tokio::fs::File::open(&file_info.path).await {
        Ok(f) => f,
        Err(e) => {
            Log::error(format!("Error opening file {}: {:?}", file_info.path, e));
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    let file_size = match file.metadata().await {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            Log::error(format!("Error getting file metadata: {:?}", e));
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let mut response = HttpResponse::Ok();
    let encoded_filename = encode_filename(&file_info.name);
    response.insert_header((
        "Content-Disposition",
        format!("attachment; filename*=UTF-8''{}", encoded_filename),
    ));
    response.content_type(file_info.content_type);
    response.insert_header(("Content-Length", file_size));

    Ok(response.streaming(tokio_util::io::ReaderStream::new(file)))
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
    use tokio::io::{AsyncReadExt, AsyncSeekExt};

    // Serve at most this many bytes per range request; clients follow up with
    // further ranges. Prevents "bytes=0-" on a large video from buffering the
    // entire file in memory.
    const MAX_CHUNK: u64 = 4 * 1024 * 1024;

    let (file_id,) = parameters.into_inner();
    Log::info(format!("Accessing stream API with file ID: {}", file_id));
    // Query the database to retrieve file details.
    let file_info = get_file_details_db(&app_state.db, file_id).await?;

    if !file_info.is_pub {
        if let Some(user_id) = session.get::<i32>("user_id").ok().flatten() {
            if file_info.user_id != user_id {
                return Ok(HttpResponse::Unauthorized()
                    .json("User Not Permitted To Access This File".to_string()));
            }
        } else {
            return Ok(HttpResponse::Unauthorized().json("File Not Permitted".to_string()));
        }
    }

    // Open the file asynchronously.
    let mut file = match tokio::fs::File::open(&file_info.path).await {
        Ok(f) => f,
        Err(e) => {
            Log::error(format!("Error opening file {}: {:?}", file_info.path, e));
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    let file_length = match file.metadata().await {
        Ok(m) => m.len(),
        Err(e) => {
            Log::error(format!("Error getting file metadata: {:?}", e));
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    if file_length == 0 {
        // Nothing to range over; previously this underflowed on `len - 1`.
        return Ok(HttpResponse::Ok()
            .content_type(file_info.content_type)
            .finish());
    }

    // Parse the Range request header.
    let range = req.headers().get("Range").and_then(|header| {
        let range_str = header.to_str().ok()?;
        range_str.strip_prefix("bytes=").map(|r| r.to_string())
    });

    let (start, requested_end) = if let Some(range) = range {
        let mut parts = range.splitn(2, '-');
        let start = parts
            .next()
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(0);
        let end = parts
            .next()
            .and_then(|e| e.parse::<u64>().ok())
            .unwrap_or(file_length - 1);
        (start, end)
    } else {
        (0, file_length - 1)
    };

    if start >= file_length || start > requested_end {
        return Ok(HttpResponse::RangeNotSatisfiable()
            .insert_header(("Content-Range", format!("bytes */{}", file_length)))
            .finish());
    }
    let end = requested_end
        .min(file_length - 1)
        .min(start + MAX_CHUNK - 1);

    // Seek and read the requested window without blocking the executor.
    file.seek(std::io::SeekFrom::Start(start)).await?;
    let length = end - start + 1;
    let mut buffer = vec![0; length as usize];
    file.read_exact(&mut buffer).await?;

    let mut response = HttpResponse::PartialContent();
    response.insert_header((
        "Content-Range",
        format!("bytes {}-{}/{}", start, end, file_length),
    ));
    response.insert_header(("Accept-Ranges", "bytes"));
    response.content_type(file_info.content_type);
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
