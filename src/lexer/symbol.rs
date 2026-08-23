use std::fmt;

use crate::lexer::{Punctuation, SpecialChar};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Punctuation(Punctuation),
    SpecialChar(SpecialChar),
}

impl TryFrom<char> for Symbol {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Ok(punctuation) = Punctuation::try_from(value) {
            return Ok(Self::Punctuation(punctuation));
        }

        if let Ok(special_char) = SpecialChar::try_from(value) {
            return Ok(Self::SpecialChar(special_char));
        }

        Err(())
    }
}

impl Symbol {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Punctuation(punctuation) => punctuation.as_str(),
            Self::SpecialChar(special_char) => special_char.as_str(),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
