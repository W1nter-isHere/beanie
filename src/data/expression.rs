use std::fmt::{Display, Formatter};
use mexprp::{Answer, Num};
use crate::data::data_type::DataType;
use crate::interpreters::math_interpreter;

#[derive(Clone, Debug)]
pub struct Expression {
    expression: String,
    value_type: DataType,
}

impl Expression {
    pub fn new(expression: String, value_type: DataType) -> Expression {
        Expression {
            expression,
            value_type,
        }
    }

    pub fn evaluate<N: Num + 'static>(&self) -> Answer<N> {
        math_interpreter::evaluate::<N>(&self.expression)
    }
    
    pub fn get_type(&self) -> DataType {
        self.value_type.clone()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} as {}", self.expression, self.value_type.to_string())
    }
}