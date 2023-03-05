use mexprp::{Answer, Num};
use crate::logger;

pub fn evaluate<N: Num + 'static>(expression: &String) -> Answer<N> {
    match mexprp::eval::<N>(expression) {
        Ok(result) => result,
        Err(_) => { 
            logger::log_error(format!("Failed to evaluate {}", expression).as_str());
            unreachable!()
        },
    }
}