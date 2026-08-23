use std::cmp::Ordering;
use thiserror::Error;

use crate::lexer::LexError;
use crate::parser::expectation::StatementKind;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ParseError {
    #[error("expected {} but got '{text}'", expected.as_str())]
    ExpectedStatement {
        depth: usize,
        expected: StatementKind,
        text: String,
    },

    #[error(
        "expected {} but got '{text}'",
        match expected.split_last() {
            Some((last, [])) => last.as_str().to_string(),
            Some((last, rest)) => format!(
                "{} or {}",
                rest.iter()
                    .map(StatementKind::as_str)
                    .collect::<Vec<_>>()
                    .join(", "),
                last.as_str()
            ),
            None => unreachable!(),
        }
    )]
    ExpectedStatements {
        depth: usize,
        expected: Vec<StatementKind>,
        text: String,
    },

    #[error("none of the parsing alternatives matched at {errors:?}")]
    NoAlternativeMatched {
        depth: usize,
        errors: Vec<ParseError>,
    },

    #[error("unexpected end of input")]
    EndOfInput,

    #[error(transparent)]
    Lex(#[from] LexError),

    #[error("could not parse {{expected.as_str()}} as {expected}")]
    ImpossibleToParse {
        depth: usize,
        expected: &'static str,
        statement: String,
    },

    #[error("{message}")]
    Custom { depth: usize, message: String },
}

impl ParseError {
    pub(crate) fn depth(&self) -> usize {
        match self {
            ParseError::ExpectedStatement { depth, .. } => *depth,
            ParseError::ExpectedStatements { depth, .. } => *depth,
            ParseError::NoAlternativeMatched { depth, .. } => *depth,
            ParseError::ImpossibleToParse { depth, .. } => *depth,
            ParseError::Custom { depth, .. } => *depth,
            ParseError::EndOfInput | ParseError::Lex(_) => usize::MAX,
        }
    }

    pub(crate) fn merge_from_option(self, other: &mut Option<ParseError>) {
        *other = Some(match other.take() {
            None => self,
            Some(prev) => prev.merge(self),
        });
    }

    pub(crate) fn merge(self, other: ParseError) -> ParseError {
        use ParseError::*;

        match self.cmp(&other) {
            Ordering::Less => return other,
            Ordering::Greater => return self,
            Ordering::Equal => {}
        }

        match (self, other) {
            (NoAlternativeMatched { depth, mut errors }, other) => {
                errors.push(other);
                NoAlternativeMatched { depth, errors }
            }

            (other, NoAlternativeMatched { depth, mut errors }) => {
                errors.push(other);
                NoAlternativeMatched { depth, errors }
            }

            (
                ExpectedStatement {
                    depth,
                    text,
                    expected: lhs,
                },
                ExpectedStatement { expected: rhs, .. },
            ) => ExpectedStatements {
                depth,
                text,
                expected: vec![lhs, rhs],
            },

            (
                ExpectedStatements {
                    depth,
                    text,
                    mut expected,
                },
                ExpectedStatement { expected: rhs, .. },
            ) => {
                expected.push(rhs);

                ExpectedStatements {
                    depth,
                    text,
                    expected,
                }
            }

            (
                ExpectedStatement {
                    depth,
                    text,
                    expected: lhs,
                },
                ExpectedStatements { mut expected, .. },
            ) => {
                expected.push(lhs);

                ExpectedStatements {
                    depth,
                    text,
                    expected,
                }
            }

            (
                ExpectedStatements {
                    depth,
                    text,
                    mut expected,
                },
                ExpectedStatements { expected: rhs, .. },
            ) => {
                expected.extend(rhs);

                ExpectedStatements {
                    depth,
                    text,
                    expected,
                }
            }

            (lhs, rhs) => NoAlternativeMatched {
                depth: lhs.depth(),
                errors: vec![lhs, rhs],
            },
        }
    }

    pub(crate) fn custom(depth: usize, message: impl Into<String>) -> Self {
        Self::Custom {
            depth,
            message: message.into(),
        }
    }
}

impl Ord for ParseError {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank = |err: &ParseError| match err {
            ParseError::ExpectedStatement { .. }
            | ParseError::ExpectedStatements { .. }
            | ParseError::NoAlternativeMatched { .. }
            | ParseError::ImpossibleToParse { .. }
            | ParseError::Custom { .. } => 0,

            ParseError::EndOfInput => 1,
            ParseError::Lex(_) => 2,
        };

        match rank(self).cmp(&rank(other)) {
            Ordering::Equal => {}
            ord => return ord,
        }

        self.depth().cmp(&other.depth())
    }
}

impl PartialOrd for ParseError {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Error)]
#[error("{message}")]
pub struct CustomParseError {
    message: String,
}

impl CustomParseError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl From<CustomParseError> for ParseError {
    fn from(err: CustomParseError) -> Self {
        ParseError::custom(0, err.message)
    }
}
