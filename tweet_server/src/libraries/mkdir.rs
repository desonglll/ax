use std::{fs, path::Path};

use colored::Colorize;

/// 在指定路径创建目录
/// 打印结果
///
/// ## 参数
///
/// * `path` - 要创建目录的路径。
///
/// ## 返回
///
/// 返回目录路径的字符串表示，如果失败则返回空字符串。
///
/// ```

pub fn make_directory(path: &str, delete_existing: bool) {
    match fs::create_dir(Path::new(path)) {
        Ok(_) => {
            println!(
                "{}{}{}",
                "Created `/{".green(),
                path.green(),
                "}` successfully.".green()
            );
        }
        Err(_) => {
            println!(
                "{}{}{}",
                "Upload Folder Exists: `/{".yellow(),
                path.yellow(),
                "}`.".yellow()
            );

            if delete_existing {
                // 删除并重新创建文件夹
                fs::remove_dir_all(&path).expect("Failed to delete the folder");
                fs::create_dir(&path).expect("Failed to create the folder");
                println!("Folder deleted and re-created.");
            } else {
                // 使用已存在的文件夹
                println!("Using the existing folder.");
            }
        }
    }
}
