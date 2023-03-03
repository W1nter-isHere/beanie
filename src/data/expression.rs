use crate::data::data_type::DataType;
use crate::interpreters::math_interpreter;

#[derive(Clone)]
pub struct Expression {
    expression: String,
}

impl Expression {
    pub fn evaluate(&self) -> DataType {
        math_interpreter::evaluate(&self.expression)
    }
    
    pub fn to_string(&self) -> String {
        self.expression.clone()
    }
    
    pub fn print(&self) {
        self.evaluate()
            .print_data();
    }
}