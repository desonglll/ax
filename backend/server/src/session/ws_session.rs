use std::sync::{Arc, Mutex};

use actix_web::{http::StatusCode, web, HttpRequest, HttpResponse, Responder};
use actix_ws::{Message, Session};
use futures::StreamExt;
use shared::lib::log::Log;

// pub async fn ws(req: HttpRequest, body: web::Payload) -> actix_web::Result<impl Responder> {
//     let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
//     Log::info("Into ws fn".to_string());

//     actix_web::rt::spawn(async move {
//         // 发送测试消息
//         if session.text("Some text").await.is_err() {
//             // session closed
//         }
//         while let Some(Ok(msg)) = msg_stream.next().await {
//             match msg {
//                 Message::Ping(bytes) => {
//                     if session.pong(&bytes).await.is_err() {
//                         return;
//                     }
//                 }
//                 Message::Text(msg) => println!("Got text: {msg}"),
//                 _ => break,
//             }
//         }

//         let _ = session.close(None).await;
//     });

//     Ok(response)
// }
#[derive(Clone)]
pub struct AppState {
    pub upload_progress: Arc<Mutex<f64>>, // Use f64 to store size_mb
}
impl AppState {
    pub fn new() -> Self {
        AppState {
            upload_progress: Arc::new(Mutex::new(0.0)),
        }
    }
}
pub async fn ws_progress(
    req: HttpRequest,
    body: web::Payload,
    upload_state: web::Data<AppState>,
) -> actix_web::Result<impl Responder> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;
    Log::info("Into ws fn".to_string());

    actix_web::rt::spawn(async move {
        // 发送测试消息
        if session
            .text(format!(
                "Uploaded: {}",
                upload_state.upload_progress.lock().unwrap()
            ))
            .await
            .is_err()
        {
            // session closed
        }
        while let Some(Ok(msg)) = msg_stream.next().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(msg) => println!("Got text: {msg}"),
                _ => break,
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

pub async fn send(mut session: Session, size: f64) -> actix_web::Result<impl Responder> {
    let resp = HttpResponse::new(StatusCode::OK);
    // 发送测试消息
    if session.text(format!("Uploaded: {}", size)).await.is_err() {
        // session closed
    }
    Ok(resp)
}
