use crate::data::contexts::beanie_context::BeanieContext;
use crate::interpreters::beanie_parser;

pub fn run(bn_file_path: String, bn_file: String, parameters: Vec<String>) {
    interpret(beanie_parser::parse(bn_file_path, bn_file, ""), parameters);
}

fn interpret(context: BeanieContext, parameters: Vec<String>) {
    let mut stripped_context = context.strip();
    let mut threads_to_wait = Vec::new();
    
    for instruction in context.instructions {
        instruction.execute(&mut stripped_context, &parameters, &mut threads_to_wait);
    }

    for thread in threads_to_wait {
        thread.join().unwrap();
    }
}