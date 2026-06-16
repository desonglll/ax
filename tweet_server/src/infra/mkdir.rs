use std::{fs, path::Path};

use colored::Colorize;

/// Create a directory at the specified path and print the result.
///
/// If the directory creation fails, it handles the failure based on the
/// value of DELETE_EXISTING.
///
/// # Parameters
///
/// * `path` - The system path where the directory is to be created.
/// * `delete_existing` - If true, any existing directory at PATH will be
///   removed and re-created.
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
                // Remove and recreate the directory.
                fs::remove_dir_all(path).expect("Failed to delete the folder");
                fs::create_dir(path).expect("Failed to create the folder");
                println!("Folder deleted and re-created.");
            } else {
                // Reuse the existing directory.
                println!("Using the existing folder.");
            }
        }
    }
}
