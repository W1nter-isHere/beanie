use std::fs;
use crate::utilities::logger;

pub fn read_file(file_path: &str) -> String {
    match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            logger::log_error(format!("Failed to read file {}", file_path).as_str());
            unreachable!()
        }
    }
}
