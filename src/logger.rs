use colored::Colorize;

pub fn log_error(text: &str) {
    println!("{}", text.red())
}

pub fn log_info(text: &str) {
    println!("{}", text)
}