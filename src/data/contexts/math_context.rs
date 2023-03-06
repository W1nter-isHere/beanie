use mexprp::{Context, Num};
use crate::data::contexts::stripped_beanie_context::StrippedBeanieContext;

pub enum MathContext<N: Num> {
    StrippedBeanie(StrippedBeanieContext),
    Math(Context<N>)
}