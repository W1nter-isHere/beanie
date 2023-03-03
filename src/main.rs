extern crate core;

mod data;
mod interpreters;

use std::{env, fs};
use colored::Colorize;
use crate::interpreters::beanie_interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // no file to run
    if args.len() <= 1 {
        println!("{}", "No file provided".red());
        return;
    }

    let file_path = &args[1];
    let file_content: String;
    
    match fs::read_to_string(file_path) {
        Ok(content) => file_content = content,
        Err(_) => {
            println!("{}", format!("Failed to read file {}", file_path).red());
            return;
        }
    }
    
    beanie_interpreter::interpret(file_content);
}
