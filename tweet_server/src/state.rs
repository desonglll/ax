use std::{collections::HashMap, sync::Mutex};

use actix_web::web;
use serde::Serialize;
use sqlx::PgPool;

use crate::libraries::dbop::get_db_pool;

/// 应用全局状态
///
/// 包含数据库连接池、请求计数和各路由响应时间记录。
pub struct AppState {
    pub db: PgPool,
    pub request_count: Mutex<u64>,
    pub response_times: Mutex<HashMap<String, Vec<u128>>>,
}

/// 应用状态响应结构
///
/// 用于 `/stats` 端点返回的统计数据。
#[derive(Serialize)]
pub struct AppStateResponse {
    pub request_count: u64,
    pub response_times: HashMap<String, Vec<u128>>,
}

impl From<AppState> for AppStateResponse {
    fn from(value: AppState) -> Self {
        AppStateResponse {
            request_count: *value.request_count.lock().unwrap(),
            response_times: value.response_times.lock().unwrap().clone(),
        }
    }
}

impl AppState {
    /// 递增请求计数器
    pub fn add_request_count(&self) {
        let mut request_count = self.request_count.lock().unwrap();
        *request_count += 1;
    }
}

/// 获取用于测试的应用状态
///
/// 创建一个连接数据库的 `AppState` 实例，请求计数从 0 开始，
/// 响应时间记录为空 Map。
///
/// # 返回值
///
/// 返回包含数据库连接池的 `web::Data<AppState>`。
pub async fn get_demo_state() -> web::Data<AppState> {
    let pool: PgPool = get_db_pool().await;
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        db: pool,
        request_count: Mutex::new(0),
        response_times: Mutex::new(HashMap::new()),
    });
    app_state
}
