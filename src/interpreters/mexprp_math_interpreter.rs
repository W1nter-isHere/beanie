use colored::Colorize;
use mexprp::{Answer, Num};

pub fn evaluate<N: Num + 'static>(expression: &String) -> Answer<N> {
    match mexprp::eval::<N>(expression) {
        Ok(result) => result,
        Err(_) => panic!("{}", format!("Failed to evaluate {}", expression).red()),
    }
}