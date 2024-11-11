use std::fs;
use std::path::Path;

pub fn create_directories(base_path: &str, directories: &[&str]) {
    for dir in directories {
        let path = format!("{}/{}", base_path, dir);
        fs::create_dir_all(&path)
            .unwrap_or_else(|_| panic!("Failed to create directory: {}", path));
    }
}

pub fn ensure_directory(path: &str) {
    if !Path::new(path).exists() {
        fs::create_dir_all(path).unwrap_or_else(|_| panic!("Failed to create directory: {}", path));
    }
}
