use crate::lexer::LexError;
use crate::parser::ParseError;
use crate::parser::expectation::StatementKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ExpectationError {
    #[error("expected one of {{expected.as_str()}} but got '{text}'")]
    ExpectedStatements {
        expected: StatementKind,
        text: String,
    },

    #[error("unexpected end of input")]
    EndOfInput,

    #[error(transparent)]
    Lex(#[from] LexError),
}

impl ExpectationError {
    pub fn into_parse_error(self, depth: usize) -> ParseError {
        match self {
            ExpectationError::ExpectedStatements { expected, text } => {
                ParseError::ExpectedStatement {
                    depth,
                    expected,
                    text,
                }
            }
            ExpectationError::EndOfInput => ParseError::EndOfInput,
            ExpectationError::Lex(err) => ParseError::Lex(err),
        }
    }
}
