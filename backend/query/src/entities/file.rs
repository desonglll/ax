use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use actix_session::Session;
use chrono::{Local, NaiveDateTime};
use diesel::dsl::insert_into;
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use shared::lib::data::Data;

use crate::{establish_pg_connection, DbPool};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "300MB")]
    pub file: TempFile,
    pub description: Text<String>,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Selectable, Debug, Default)]
#[diesel(table_name = crate::schema::files)]
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
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertFileRequest {
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
}

impl File {
    pub fn new(
        session: &Session,
        name: String,
        size: i64,
        content_type: String,
        description: Option<String>,
        checksum: String,
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
        }
    }
    pub fn insert_file(&self, pool: &DbPool) -> Result<Data<File>, diesel::result::Error> {
        let mut conn = establish_pg_connection(pool).unwrap();
        // 将文件信息插入数据库
        let result = insert_into(crate::schema::files::table)
            .values(self)
            .returning(File::as_returning())
            .get_result(&mut conn)?;

        // 返回保存的文件对象
        Ok(Data::new(result, None))
    }
    pub fn get_file(pool: &DbPool, id: Uuid) -> Result<Data<File>, diesel::result::Error> {
        use crate::schema::files::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = dsl::files.filter(dsl::id.eq(id)).first(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
    pub fn delete_file_by_path(
        pool: &DbPool,
        path: String,
    ) -> Result<Data<File>, diesel::result::Error> {
        use crate::schema::files::dsl;
        use diesel::prelude::*;
        let mut conn = establish_pg_connection(pool).unwrap();
        let data = diesel::delete(dsl::files)
            .filter(dsl::path.eq(path))
            .get_result::<File>(&mut conn)?;
        let body = Data::new(data, None);
        Ok(body)
    }
}
