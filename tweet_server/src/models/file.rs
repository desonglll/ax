use actix_session::Session;
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct File {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub content_type: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub user_id: i32,
    pub description: Option<String>,
    pub checksum: String,
    pub is_deleted: bool,
    pub is_pub: bool,
}

impl File {
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
        let path = format!("{}/upload/{}", base_url, name);
        Self {
            id: Uuid::new_v4(),
            name,
            path,
            size,
            content_type,
            created_at: Some(Local::now().naive_local()),
            updated_at: Some(Local::now().naive_local()),
            user_id,
            description,
            checksum,
            is_deleted: false,
            is_pub,
        }
    }
}
#[derive(Deserialize)]
pub struct FileFilter {
    pub name: Option<String>,
    pub path: Option<String>,
    pub user_id: Option<i32>,
    pub is_deleted: Option<bool>,
    pub is_pub: Option<bool>,
}
