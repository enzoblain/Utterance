use crate::lexer::{Lexer, Number, Symbol, Token, TokenKind, TokenType, Word};
use crate::parser::expectation::{
    ExpectationError, Match, NumberKind, Statement, StatementKind, WordKind,
};
use crate::syntax::HighlightKind;

fn peek_token_or_error<'input, const P: usize>(
    lexer: &mut Lexer<'input, P>,
    n: usize,
) -> Result<Token<'input>, ExpectationError> {
    match lexer.peek_token(n) {
        Some(Ok(token)) => Ok(token.clone()),
        Some(Err(err)) => Err(ExpectationError::Lex(err.clone())),
        None => Err(ExpectationError::EndOfInput),
    }
}

fn expect_kinds(token: &Token, expected: &[TokenKind]) -> bool {
    expected.contains(&token.token_type().kind())
}

pub(crate) fn expect_capitalize_word<'a, const P: usize>(
    lexer: &mut Lexer<P>,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    if let TokenType::Word(w) = token.token_type()
        && w.chars().next().unwrap().is_uppercase()
    {
        Ok(Match::new(0, Statement::Noop))
    } else {
        Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::CapitalizeWord,
            text: token.to_string(),
        })
    }
}

pub(crate) fn expect_word<'input, const P: usize>(
    lexer: &mut Lexer<'input, P>,
    kind: HighlightKind,
) -> Result<Match<'input>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    match token.token_type() {
        TokenType::Word(word) => Ok(Match::new(1, Statement::Word(Word::Custom { word, kind }))),
        _ => Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Word(WordKind::Custom { kind }),
            text: token.to_string(),
        }),
    }
}

pub(crate) fn expect_exact_word<'input, const P: usize>(
    lexer: &mut Lexer<'input, P>,
    expected: &'static str,
) -> Result<Match<'input>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    if let TokenType::Word(w) = token.token_type()
        && w == &expected
    {
        Ok(Match::new(1, Statement::Word(Word::Exact(expected))))
    } else {
        Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Word(WordKind::Exact(expected)),
            text: token.to_string(),
        })
    }
}

pub(crate) fn expect_one_of_words<'input, const P: usize>(
    lexer: &mut Lexer<'input, P>,
    expected: &'static [&'static str],
) -> Result<Match<'input>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    if let TokenType::Word(w) = token.token_type()
        && let Some(expected) = expected.iter().find(|e| *e == w)
    {
        return Ok(Match::new(1, Statement::Word(Word::Exact(expected))));
    }

    Err(ExpectationError::ExpectedStatements {
        expected: StatementKind::OneOfExactWords(expected),
        text: token.to_string(),
    })
}

pub(crate) fn expect_comment<'a, const P: usize>(
    lexer: &mut Lexer<P>,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    if StatementKind::from(token.token_type()) != *lexer.comment_symbol() {
        return Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Comment,
            text: token.to_string(),
        });
    }

    let (n, _) = lexer.peek_until_kind(&[TokenKind::Newline, TokenKind::End])?;

    Ok(Match::new(n, Statement::Comment))
}

pub(crate) fn expect_number<'a, const P: usize>(
    lexer: &mut Lexer<P>,
    expected: NumberKind,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    match token.token_type() {
        TokenType::Number(number) => {
            let mut number = *number;

            if !number.try_convert_into(expected) {
                return Err(ExpectationError::ExpectedStatements {
                    expected: StatementKind::Number(expected),
                    text: token.to_string(),
                });
            }

            Ok(Match::new(1, Statement::Number(number)))
        }

        _ => Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Number(expected),
            text: token.to_string(),
        }),
    }
}

pub(crate) fn expect_exact_number<'a, const P: usize>(
    lexer: &mut Lexer<P>,
    expected: Number,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    match token.token_type() {
        TokenType::Number(number) => {
            let mut number = *number;

            if !number.try_convert_into(expected.into()) {
                return Err(ExpectationError::ExpectedStatements {
                    expected: StatementKind::Number(expected.into()),
                    text: token.to_string(),
                });
            }

            if number == expected {
                Ok(Match::new(1, Statement::Number(expected)))
            } else {
                Err(ExpectationError::ExpectedStatements {
                    expected: StatementKind::Number(expected.into()),
                    text: token.to_string(),
                })
            }
        }

        _ => Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Number(expected.into()),
            text: token.to_string(),
        }),
    }
}

pub(crate) fn expect_new_line_or_end<'a, const P: usize>(
    lexer: &mut Lexer<P>,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    if !expect_kinds(&token, &[TokenKind::Newline, TokenKind::End]) {
        return Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::NewLineOrEnd,
            text: token.to_string(),
        });
    }

    Ok(Match::new(1, Statement::NewLineOrEnd))
}

pub(crate) fn expect_noop<'a, const P: usize>(
    _: &mut Lexer<P>,
) -> Result<Match<'a>, ExpectationError> {
    Ok(Match::new(0, Statement::Noop))
}

pub(crate) fn expect_symbol<'a, const P: usize>(
    lexer: &mut Lexer<P>,
    expected: Symbol,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    match token.token_type() {
        TokenType::Symbol(symbol) if *symbol == expected => {
            Ok(Match::new(1, Statement::Symbol(expected)))
        }
        _ => Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::Symbol(expected),
            text: token.to_string(),
        }),
    }
}

pub(crate) fn expect_double_symbol<'a, const P: usize>(
    lexer: &mut Lexer<P>,
    expected: Symbol,
) -> Result<Match<'a>, ExpectationError> {
    let token = peek_token_or_error(lexer, 0)?;

    match token.token_type() {
        TokenType::DoubleSymbol(symbol) if *symbol == expected => {
            Ok(Match::new(1, Statement::DoubleSymbol(expected)))
        }
        _ => Err(ExpectationError::ExpectedStatements {
            expected: StatementKind::DoubleSymbol(expected),
            text: token.to_string(),
        }),
    }
}
