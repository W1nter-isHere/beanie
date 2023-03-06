use pest::iterators::{Pair, Pairs};
use pest::Parser;
use crate::data::expression::data_type::DataType;
use crate::data::expression::BeanieExpression;
use crate::DEFAULT_DATA_TYPE;
use crate::utilities::logger;

#[derive(Parser)]
#[grammar = "syntax/expression_v0.21.pest"]
struct ExpressionParser;

pub fn parse(expression_text: String, suffix: &str) -> BeanieExpression {
    let s: &'static str = Box::leak(expression_text.into_boxed_str());
    match ExpressionParser::parse(Rule::expression, s) {
        Ok(mut expression) => unsafe {
            let mut expression_components = expression.next().unwrap().into_inner(); 
            let math_expression_original = expression_components.next().unwrap();
            let datatype = get_as_datatype(&mut expression_components, &DEFAULT_DATA_TYPE);
            
            if suffix.is_empty() {
                return BeanieExpression::Math(math_expression_original, datatype);
            }
            
            let mut variable_spans = Vec::new();

            for math_component in math_expression_original.clone().into_inner().flatten() {
                if math_component.as_rule() == Rule::variable_name {
                    variable_spans.push(math_component)
                }
            }

            let org_expr_str = math_expression_original.as_str().to_string();

            let mut new_expr_str = org_expr_str.clone();
            let mut offset = 0;

            for span in variable_spans {
                let start = span.as_span().start();
                let end = start + span.as_str().trim().len();
                new_expr_str.replace_range((start + offset)..(end + offset), (String::from(&org_expr_str[start..end]) + "_" + suffix).as_str());
                offset += suffix.len() + 1;
            }
            
            return BeanieExpression::Math(parse(new_expr_str, "").get_math().unwrap(), datatype);
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
                let data_type = expression.next().unwrap();
                data_type.as_str().trim().parse::<DataType>().unwrap()
            } else {
                logger::log_error("Math expression not followed by as keyword");
                unreachable!()
            }
        }
        None => default_data_type.clone(),
    }
}