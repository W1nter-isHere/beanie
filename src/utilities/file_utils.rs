use std::fs;
use rand::Rng;
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

pub fn random_suffix() -> String {
    let mut result = String::new();
    
    for _ in 0..16 {
        result.push(rand::thread_rng().gen_range(97..123) as u8 as char);
    }

    result
}