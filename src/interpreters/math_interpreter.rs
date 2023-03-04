use mexprp::{Answer, Num};
use crate::interpreters::mexprp_math_interpreter;

// this exist is in case I want to implement my own math interpreter in the future for whatever reason
pub fn evaluate<N: Num + 'static>(expression: &String) -> Answer<N> {
    mexprp_math_interpreter::evaluate::<N>(expression)
}