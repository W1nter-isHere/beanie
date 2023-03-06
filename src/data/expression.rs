use mexprp::{Answer, Context, Expression, MathError, Num, Term};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use pest::iterators::Pair;
use rug::{Complex, Rational};
use crate::data::beanie_context::{BeanieContext, StrippedBeanieContext};
use crate::data::expression::data_type::DataType;
use crate::data::expression::expression_type::ExpressionType;
use crate::interpreters::expression_parser::Rule;

pub mod data_type;
pub mod expression_type;

#[derive(Clone, Debug)]
pub enum BeanieExpression {
    Math(Pair<'static, Rule>, DataType),
    Boolean(bool),
    FilePath(String),
    DataType(DataType),
}

impl BeanieExpression {
    pub fn construct_math_context<N: Num + Clone + 'static>(&self, expr: &Pair<Rule>, bn_context: &StrippedBeanieContext) -> Context<N> {
        let mut math_context: Context<N> = Context::new();

        for component in expr.clone().into_inner().flatten() {
            if component.as_rule() == Rule::variable_name {
                let name = component.as_str().to_string();
                
                if bn_context.has_constant(&name) {
                    let constant_expression = bn_context.get_constant(&name).unwrap();
                    let result = constant_expression.0.evaluate::<N>(bn_context);
                    let value = match result {
                        Answer::Single(answer) => answer,
                        Answer::Multiple(answers) => answers[constant_expression.1].clone(),
                    };
                    math_context.set_var(&name, value);
                } else if bn_context.has_function(&name) {
                    math_context.set_func(&name, bn_context.get_function(&name).unwrap());
                }
            } 
        }
        
        math_context
    }

    pub fn evaluate<N: Num + 'static>(&self, bn_context: &StrippedBeanieContext) -> Answer<N> {
        match self {
            BeanieExpression::Math(expr, _) => Expression::parse_ctx(expr.as_str().trim(), self.construct_math_context::<N>(expr, bn_context)).unwrap().eval().unwrap(),
            _ => unreachable!()
        }
    }

    pub fn evaluate_with_math_ctx<N: Num + 'static>(&self, math_context: Context<N>) -> Answer<N> {
        match self {
            BeanieExpression::Math(expr, _) => Expression::parse_ctx(expr.as_str().trim(), math_context).unwrap().eval().unwrap(),
            _ => unreachable!()
        }
    }
    
    pub fn evaluate_to_string(&self, context: &StrippedBeanieContext) -> String {
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
    
    pub fn get_math(&self) -> Option<Pair<'static, Rule>> {
        match self {
            BeanieExpression::Math(math, _) => Some(math.clone()),
            _ => None,
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
