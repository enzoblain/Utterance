use std::fmt;

use crate::parser::expectation::NumberKind;

#[derive(Debug, Copy, Clone)]
pub enum Number {
    UnsignedInteger(usize),
    Integer(isize),
    Float(f64),
}

impl Number {
    pub(crate) fn try_convert_into(&mut self, kind: NumberKind) -> bool {
        let converted = match (*self, kind) {
            (Number::UnsignedInteger(n), NumberKind::Integer) => Some(Number::Integer(n as isize)),
            (Number::UnsignedInteger(n), NumberKind::Float) => Some(Number::Float(n as f64)),
            (Number::Integer(n), NumberKind::Float) => Some(Number::Float(n as f64)),

            (number, NumberKind::UnsignedInteger)
                if matches!(number, Number::UnsignedInteger(_)) =>
            {
                Some(number)
            }

            (number, NumberKind::Integer) if matches!(number, Number::Integer(_)) => Some(number),
            (number, NumberKind::Float) if matches!(number, Number::Float(_)) => Some(number),

            _ => None,
        };

        if let Some(value) = converted {
            *self = value;
            true
        } else {
            false
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::UnsignedInteger(a), Self::UnsignedInteger(b)) => a == b,
            (Self::Integer(a), Self::Integer(b)) => a == b,
            (Self::Float(a), Self::Float(b)) => a.to_bits() == b.to_bits(),

            _ => false,
        }
    }
}

impl Eq for Number {}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsignedInteger(value) => write!(f, "unsigned integer: {value}"),
            Self::Integer(value) => write!(f, "integer: {value}"),
            Self::Float(value) => write!(f, "float: {value}"),
        }
    }
}

impl From<Number> for NumberKind {
    fn from(number: Number) -> Self {
        match number {
            Number::UnsignedInteger(_) => NumberKind::UnsignedInteger,
            Number::Integer(_) => NumberKind::Integer,
            Number::Float(_) => NumberKind::Float,
        }
    }
}
