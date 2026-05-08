use actix_web::{web, HttpResponse};

use crate::state::{AppState, AppStateResponse};

/// 获取应用状态统计信息
pub async fn get_stats(app_state: web::Data<AppState>) -> HttpResponse {
    let request_count = *app_state.request_count.lock().unwrap();
    let response_times = app_state.response_times.lock().unwrap().clone();

    let stats = AppStateResponse {
        request_count,
        response_times,
    };

    HttpResponse::Ok().json(stats)
}
