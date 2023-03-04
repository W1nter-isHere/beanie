use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::keywords;

#[derive(Clone, Debug)]
pub enum DataType {
    ComplexStruct,
    ComplexFloat,
    Rational,
    ComplexRational,
    Irrational,
}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for DataType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            keywords::types::DECIMAL => Ok(DataType::ComplexStruct),
            keywords::types::IMAGINARY_DECIMAL => Ok(DataType::ComplexFloat),
            keywords::types::FRACTION => Ok(DataType::Rational),
            keywords::types::IMAGINARY_FRACTION => Ok(DataType::ComplexRational),
            keywords::types::IRRATIONAL => Ok(DataType::Irrational),
            _ => Err(())
        }
    }
}