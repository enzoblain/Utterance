use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Punctuation {
    Comma,
    SemiColon,
    Colon,
    Point,
    Exclamation,
    Question,
}

impl TryFrom<char> for Punctuation {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            ',' => Ok(Self::Comma),
            ';' => Ok(Self::SemiColon),
            ':' => Ok(Self::Colon),
            '.' => Ok(Self::Point),
            '!' => Ok(Self::Exclamation),
            '?' => Ok(Self::Question),
            _ => Err(()),
        }
    }
}

impl Punctuation {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Comma => ",",
            Self::SemiColon => ";",
            Self::Colon => ":",
            Self::Point => ".",
            Self::Exclamation => "!",
            Self::Question => "?",
        }
    }
}

impl fmt::Display for Punctuation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
