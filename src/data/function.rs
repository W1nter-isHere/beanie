#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Function {
    name: String,
    parameters: Vec<String>,
}

impl Function {
    pub fn new(name: String, parameters: Vec<String>) -> Function {
        Function {
            name,
            parameters
        }
    }
}