use mexprp::{Answer, Context, Expression, Num};
use mexprp::num::{ComplexFloat, ComplexRugRat};
use pest::iterators::Pair;
use rug::{Complex, Rational};
use crate::data::contexts::math_context::MathContext;
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;
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
        let mut math_context: Context<N> = Context::empty();

        for component in expr.clone().into_inner().flatten() {
            if component.as_rule() == Rule::variable_name {
                let name = component.as_str().trim().to_string();
                if math_context.vars.contains_key(&name) || math_context.funcs.contains_key(&name) {
                    continue;
                } 
                
                if bn_context.has_constant(&name) {
                    let constant_expression = bn_context.get_constant(&name).unwrap();
                    let result = constant_expression.0.evaluate::<N>(MathContext::StrippedBeanie::<N>(bn_context.clone()));
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

    pub fn evaluate<N: Num + 'static>(&self, ctx: MathContext<N>) -> Answer<N> {
        match self {
            BeanieExpression::Math(expr, _) => {
                let str = expr.as_str().trim();
                match ctx {
                    MathContext::StrippedBeanie(bn_context) => Expression::parse_ctx(str, self.construct_math_context::<N>(expr, &bn_context)).unwrap().eval().unwrap(),
                    MathContext::Math(math_context) => Expression::parse_ctx(str, math_context).unwrap().eval().unwrap(),
                }
            },
            _ => unreachable!()
        }
    }
    
    pub fn evaluate_to_string(&self, context: &StrippedBeanieContext) -> String {
        match self {
            BeanieExpression::Math(expression_component, data_type) => {
                match data_type {
                    DataType::Decimal => self.evaluate::<f64>(MathContext::StrippedBeanie::<f64>(context.clone())).to_string(),
                    DataType::ImaginaryDecimal => self.evaluate::<ComplexFloat>(MathContext::StrippedBeanie::<ComplexFloat>(context.clone())).to_string(),
                    DataType::Complex => self.evaluate::<Complex>(MathContext::StrippedBeanie::<Complex>(context.clone())).to_string(),
                    DataType::Rational => self.evaluate::<Rational>(MathContext::StrippedBeanie::<Rational>(context.clone())).to_string(),
                    DataType::ComplexRational => self.evaluate::<ComplexRugRat>(MathContext::StrippedBeanie::<ComplexRugRat>(context.clone())).to_string(),
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
