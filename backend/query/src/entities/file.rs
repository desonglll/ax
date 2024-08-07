use std::fs;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use actix_multipart::form::MultipartForm;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_session::Session;
use chrono::{Local, NaiveDateTime};
use diesel::{Insertable, Queryable, RunQueryDsl, Selectable, SelectableHelper};
use diesel::dsl::insert_into;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

use shared::lib::data::Data;

use crate::{DbPool, establish_pg_connection};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(limit = "100MB")]
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
pub struct InsertFileRequest {
    pub name: String,
    pub user_id: i32,
    pub description: Option<String>,
}

impl File {
    pub fn new(session: &Session, upload_form: &mut UploadForm) -> Self {
        let user_id = session.get::<i32>("user_id").unwrap().unwrap_or(-1);
        let description = upload_form.description.clone().clone();
        let base_url: String = std::env::current_dir().unwrap().to_str().unwrap().to_string();
        let upload_folder = format!("{}/upload", base_url);
        Self {
            id: Uuid::new_v4(),
            name: upload_form.file.file_name.clone().unwrap(),
            path: File::get_save_path(upload_folder, &upload_form.file).to_str().unwrap().to_string(),
            size: upload_form.file.size as i64,
            content_type: upload_form.file.content_type.clone().unwrap().to_string(),
            created_at: Some(Local::now().naive_local()),
            updated_at: Some(Local::now().naive_local()),
            user_id,
            description: Some(description),
            checksum: Self::calculate_hash(&mut upload_form.file),
            is_deleted: false,
        }
    }

    pub fn save(&self, pool: &DbPool, session: &Session, mut upload_form: UploadForm) -> Result<Data<File>, Box<dyn std::error::Error>
    > {
        let mut conn = establish_pg_connection(pool).unwrap();
        // 获取保存路径
        let save_path = Path::new(&self.path);

        // 创建目录（如果不存在）
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // 打开目标文件进行写入
        let mut file = fs::File::create(&save_path)?;

        // 从 NamedTempFile 读取内容并写入到目标文件
        let mut temp_file = upload_form.file.file.as_file(); // 获取文件的可读引用
        // 重置文件指针到文件开头
        temp_file.seek(SeekFrom::Start(0))?;
        let mut buffer = Vec::new();
        let bytes_read = temp_file.read_to_end(&mut buffer)?; // 读取临时文件内容到缓冲区

        if bytes_read == 0 {
            println!("Warning: No data read from temp file");
        } else {
            println!("Data read: {} bytes", bytes_read);
        }

        file.write_all(&buffer)?; // 将缓冲区内容写入目标文件

        let insert_file = File::new(session, &mut upload_form);
        // 将文件信息插入数据库
        let result = insert_into(crate::schema::files::table)
            .values(&insert_file)
            .returning(File::as_returning())
            .get_result(&mut conn)?;

        // 返回保存的文件对象
        Ok(Data::new(result, None))
    }

    // 计算文件哈希值
    pub fn calculate_hash(file: &mut TempFile) -> String {
        let mut hasher = Sha256::new();
        let mut buffer = Vec::new();
        file.file.read_to_end(&mut buffer).unwrap();
        hasher.update(buffer);
        format!("{:x}", hasher.finalize())
    }

    // 获取保存路径
    pub fn get_save_path(base_path: String, file: &TempFile) -> PathBuf {
        let file_name = file.file_name.clone().unwrap();
        let mut path = PathBuf::from(base_path);
        path.push(file_name);
        path
    }
}

#[cfg(test)]
mod file_tests {
    use std::fs::File;

    use actix_web::http::header::{ContentDisposition, DispositionParam, DispositionType};

    use super::*;

    #[test]
    fn test_get_filename_with_valid_filename() {
        // 构建一个包含有效文件名的 ContentDisposition
        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(String::from("test_file.txt"))],
        };

        // 调用 get_filename 函数
        let filename = File::get_filename(&content_disposition);

        // 断言文件名是我们期望的文件名
        assert_eq!(filename, "test_file.txt");
    }

    #[test]
    fn test_get_filename_with_missing_filename() {
        // 构建一个不包含文件名的 ContentDisposition
        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![], // 没有 Filename 参数
        };

        // 调用 get_filename 函数
        let filename = File::get_filename(&content_disposition);

        // 断言文件名是默认文件名
        assert_eq!(filename, "default_filename");
    }

    #[test]
    fn test_get_file_type_with_valid_filename() {
        // 构建一个包含有效文件名的 ContentDisposition
        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![DispositionParam::Filename(String::from("test_file.txt"))],
        };

        // 调用 get_file_type 函数
        let file_type = File::get_file_type(&content_disposition);

        // 断言文件类型是我们期望的文件类型
        assert_eq!(file_type, "test_file.txt");
    }

    #[test]
    fn test_get_file_type_with_missing_filename() {
        // 构建一个不包含文件名的 ContentDisposition
        let content_disposition = ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![], // 没有 Filename 参数
        };

        // 调用 get_file_type 函数
        let file_type = File::get_file_type(&content_disposition);

        // 断言文件类型是 "unknown"
        assert_eq!(file_type, "unknown");
    }

    #[test]
    fn test_calculate_hash() {
        use std::io::Write;
        // 创建一个临时文件并写入数据
        let temp_file_path = PathBuf::from("./temp_file.txt");
        let mut file = File::create(&temp_file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // 调用 calculate_hash 函数
        let hash = File::calculate_hash(&temp_file_path);

        // 断言哈希值是我们期望的值
        let expected_hash = "d9014c4624844aa5bac314773d6b689ad467fa4e1d1a50a1b8a99d5a95f72ff5";
        assert_eq!(hash, expected_hash);

        // 删除临时文件
        fs::remove_file(temp_file_path).unwrap();
    }

    #[test]
    fn test_get_save_path() {
        // 调用 get_save_path 函数
        let filename = "test_file.txt";
        let base_url: String = std::env::var("CRAB_ROCKET_WORKING_DIR").unwrap();
        let save_path = File::get_save_path(base_url.clone(), filename);

        // 构造期望的路径
        let mut expected_path = PathBuf::from(base_url.clone());
        expected_path.push(filename);

        // 断言生成的路径是我们期望的路径
        assert_eq!(save_path, expected_path);
    }
}
