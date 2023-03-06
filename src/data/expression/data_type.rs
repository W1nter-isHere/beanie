use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::utilities::keywords;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataType {
    Decimal,
    ImaginaryDecimal,
    Complex,
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
            keywords::types::DECIMAL => Ok(DataType::Decimal),
            keywords::types::IMAGINARY_DECIMAL => Ok(DataType::ImaginaryDecimal),
            keywords::types::COMPLEX => Ok(DataType::Complex),
            keywords::types::FRACTION => Ok(DataType::Rational),
            keywords::types::IMAGINARY_FRACTION => Ok(DataType::ComplexRational),
            keywords::types::IRRATIONAL => Ok(DataType::Irrational),
            _ => Err(())
        }
    }
}