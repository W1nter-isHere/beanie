use pest::iterators::Pairs;
use pest::Parser;
use crate::data::data_type::DataType;
use crate::data::expression::Expression;
use crate::{DEFAULT_DATA_TYPE, logger};

#[derive(Parser)]
#[grammar = "syntax/grammar/expression_v0.1.pest"]
struct ExpressionParser;

pub fn parse(expression_text: String) -> Expression {
    match ExpressionParser::parse(Rule::expression, expression_text.as_str()) {
        Ok(mut expression_components) => {
            let math_expression = expression_components.next().unwrap();
            let datatype = get_as_datatype(&mut expression_components, &DEFAULT_DATA_TYPE);
            Expression::new(math_expression, datatype)
        }
        Err(err) => { 
            logger::log_error(format!("{}", err).as_str());
            unreachable!()
        }
    }
}

fn get_as_datatype(expression: &mut Pairs<Rule>, default_data_type: &DataType) -> DataType {
    match expression.next() {
        Some(assumed_as_keyword) => {
            if assumed_as_keyword.as_rule() == Rule::as_keyword {
                expression.next().unwrap().as_str().trim().parse::<DataType>().unwrap()
            } else {
                logger::log_error("Math expression not followed by as keyword");
                unreachable!()
            }
        }
        None => default_data_type.clone(),
    }
}