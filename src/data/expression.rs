use mexprp::{Answer, Num};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use pest::iterators::Pair;
use rug::{Complex, Rational};
use crate::data::beanie_context::BeanieContext;
use crate::data::data_type::DataType;
use crate::data::expression_type::ExpressionType;
use crate::interpreters::expression_parser::Rule;
use crate::interpreters::math_interpreter;

#[derive(Clone, Debug)]
pub enum BeanieExpression {
    Math(Pair<'static, Rule>, DataType),
    Boolean(bool),
    FilePath(String),
    DataType(DataType),
}

impl BeanieExpression {
    pub fn evaluate<N: Num + 'static>(&self, context: &BeanieContext) -> Answer<N> {
        match self{
            BeanieExpression::Math(_, _) => {
                
            },
            _ => unreachable!()
        };
        
        math_interpreter::evaluate::<N>(&String::from(""))
    }
    
    pub fn evaluate_to_string(&self, context: &BeanieContext) -> String {
        match self {
            BeanieExpression::Math(expression_component, data_type) => {
                match data_type {
                    DataType::Decimal => self.evaluate::<f64>(context).to_string(),
                    DataType::ImaginaryDecimal => self.evaluate::<ComplexFloat>(context).to_string(),
                    DataType::Complex => self.evaluate::<Complex>(context).to_string(),
                    DataType::Rational => self.evaluate::<Rational>(context).to_string(),
                    DataType::ComplexRational => self.evaluate::<ComplexRugRat>(context).to_string(),
                    _ => expression_component.as_str().to_string(),
                }
            } 
            BeanieExpression::Boolean(b) => b.to_string(),
            BeanieExpression::FilePath(file_path) => file_path.to_string(),
            BeanieExpression::DataType(data_type) => data_type.to_string(),
        }
    }
    
    pub fn get_type(&self) -> ExpressionType {
        match self {
            BeanieExpression::Math(_, _) => ExpressionType::MathExpression,
            BeanieExpression::Boolean(_) => ExpressionType::Boolean,
            BeanieExpression::FilePath(_) => ExpressionType::FilePath,
            BeanieExpression::DataType(_) => ExpressionType::DataType,
        }
    }
}