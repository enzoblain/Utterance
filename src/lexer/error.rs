use thiserror::Error;

#[derive(Debug, Error, Clone, Eq, PartialEq)]
pub enum LexError {
    #[error("unexpected character '{character}' at position {position}")]
    UnexpectedCharacter { position: usize, character: char },

    #[error(
        "non-ASCII symbol '{character}' at position {position} must not be preceded by whitespace"
    )]
    NonAsciiPrecededByWhitespace { position: usize, character: char },

    #[error("non-ASCII symbol '{character}' at position {position} must be followed by whitespace")]
    NonAsciiNotFollowedByWhitespace { position: usize, character: char },
}
