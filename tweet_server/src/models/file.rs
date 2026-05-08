use actix_session::Session;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 文件数据模型
///
/// 对应数据库 `files` 表的记录，表示一个上传的文件。
#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: Option<DateTime<chrono::Utc>>,
    pub updated_at: Option<DateTime<chrono::Utc>>,
    pub user_id: i32,
    pub description: Option<String>,
    pub checksum: String,
    pub is_deleted: bool,
    pub is_pub: bool,
}

impl File {
    /// 创建新的文件模型实例
    ///
    /// 从 session 中获取用户 ID，根据文件名生成保存路径，
    /// 自动生成 UUID 和时间戳。
    ///
    /// # 参数
    ///
    /// - `session`: 请求的 session 对象，用于获取用户 ID
    /// - `name`: 文件名
    /// - `size`: 文件大小（字节）
    /// - `content_type`: 文件 MIME 类型
    /// - `description`: 文件描述
    /// - `checksum`: 文件校验和（SHA-256）
    /// - `is_pub`: 文件是否公开
    pub fn new(
        session: &Session,
        name: String,
        size: i64,
        content_type: String,
        description: Option<String>,
        checksum: String,
        is_pub: bool,
    ) -> Self {
        let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(-1);
        let base_url: String = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let path = format!("{}/uploads/{}", base_url, name);
        Self {
            id: Uuid::new_v4(),
            name,
            path,
            size,
            content_type,
            created_at: Some(Local::now().to_utc()),
            updated_at: Some(Local::now().to_utc()),
            user_id,
            description,
            checksum,
            is_deleted: false,
            is_pub,
        }
    }
}

/// 文件筛选条件
///
/// 用于查询文件列表时的筛选参数。
#[derive(Deserialize)]
pub struct FileFilter {
    pub name: Option<String>,
    pub path: Option<String>,
    pub user_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub is_pub: Option<bool>,
}
