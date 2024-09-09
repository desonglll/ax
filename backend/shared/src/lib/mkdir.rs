use std::{fs, io, path::Path};

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

pub fn make_directory(path: &str) {
    match fs::create_dir(Path::new(path)) {
        Ok(_) => {
            println!(
                "{}{}{}",
                "Created `/{".green(),
                path.green(),
                "}` successfully.".green()
            );
        }
        Err(_err) => {
            println!(
                "{}{}{}",
                "Upload Folder Exists: `/{".yellow(),
                path.yellow(),
                "}`.".yellow()
            );
            loop {
                // 提示用户输入
                println!("Do you want to delete the existing folder and create a new one? (Enter for Yes / n for No)");

                // 读取用户输入
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                let input = input.trim(); // 去除输入的换行符

                // 根据用户输入执行相应操作
                match input.to_lowercase().as_str() {
                    "" => {
                        // 删除并重新创建文件夹
                        fs::remove_dir_all(path).expect("Failed to delete the folder");
                        fs::create_dir(path).expect("Failed to create the folder");
                        println!("Folder deleted and re-created.");
                        break;
                    }
                    "n" => {
                        // 使用已存在的文件夹
                        println!("Using the existing folder.");
                        break;
                    }
                    _ => {
                        println!("Invalid input. Please enter `Y` to confirm, `n` to cancel, or press Enter to re-confirm.");
                    }
                }
            }
        }
    }
}
