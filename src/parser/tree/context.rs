use std::fmt::Debug;

use crate::parser::CustomParseError;

pub trait ParseContext: Clone + Default + Debug {
    type Error: Into<CustomParseError>;
}
