use std::fmt::{Display, Formatter};
use mexprp::{Answer, Num};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use pest::iterators::Pair;
use rug::{Complex, Rational};
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::interpreters::expression_parser::Rule;
use crate::interpreters::math_interpreter;

#[derive(Clone, Debug)]
pub struct Expression {
    expression: Pair<'static, Rule>,
    value_type: DataType,
}

impl Expression {
    pub fn new(expression: Pair<Rule>, value_type: DataType) -> Expression {
        Expression {
            expression,
            value_type,
        }
    }

    pub fn evaluate<N: Num + 'static>(&self, context: &BeanieContext) -> Answer<N> {
        let mut evaluated_string = self.expression.clone();
        
        
        math_interpreter::evaluate::<N>(&String::from(""))
    }
    
    pub fn evaluate_to_string(&self, context: &BeanieContext) -> String {
        match self.value_type {
            DataType::ComplexStruct => self.evaluate::<Complex>(context).to_string(),
            DataType::ComplexFloat => self.evaluate::<ComplexFloat>(context).to_string(),
            DataType::Rational => self.evaluate::<Rational>(context).to_string(),
            DataType::ComplexRational => self.evaluate::<ComplexRugRat>(context).to_string(),
            _ => self.expression.to_string(),
        }
    }
    
    pub fn get_type(&self) -> DataType {
        self.value_type.clone()
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expression)
    }
}