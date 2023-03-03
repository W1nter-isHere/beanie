use std::collections::HashMap;
use crate::data::beanie_context::BeanieContext;

pub fn interpret(file_content: String) {
    let lines: Vec<&str> = file_content.split("\n").collect();
    let context = create_context(&lines);
}

fn create_context(lines: &Vec<&str>) -> BeanieContext {
    let expressions= HashMap::new();
    let instructions= Vec::new();
    
    for line in lines {
        for char in line.chars() {
            
        }
    }
    
    BeanieContext {
        expressions,
        instructions,
    }
}

fn get_result<T>(context: &BeanieContext) -> Option<T> {
    None
}