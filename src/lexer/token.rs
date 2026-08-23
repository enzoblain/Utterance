use std::fmt;

use crate::lexer::{Number, TokenType};
use crate::syntax::Span;

#[derive(Clone, Debug)]
pub struct Token<'a> {
    span: Span,
    token_type: TokenType<'a>,
}

impl<'a> Token<'a> {
    pub(crate) fn new(position: usize, token: &'a str) -> Self {
        let token_type = TokenType::Word(token);
        let span = Span::new(position, position + token.len());

        Self { span, token_type }
    }

    pub(crate) fn new_number(position: usize, number: Number, len: usize) -> Self {
        let span = Span::new(position, position + len);

        Self {
            span,
            token_type: TokenType::Number(number),
        }
    }

    pub(crate) fn new_hardcoded(position: usize, token_type: TokenType<'a>) -> Self {
        let len = match token_type {
            TokenType::End | TokenType::Newline => 0,

            TokenType::Symbol(_) => 1,
            TokenType::DoubleSymbol(_) => 2,

            TokenType::Word(_) | TokenType::Number(_) => unreachable!(),
        };

        let span = Span::new(position, position + len);

        Self { span, token_type }
    }

    pub(crate) fn token_type(&self) -> &TokenType<'a> {
        &self.token_type
    }

    pub(crate) fn span(&self) -> Span {
        self.span
    }
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.token_type {
            TokenType::Word(text) => write!(f, "{text}"),
            TokenType::Number(value) => write!(f, "{value}"),
            TokenType::Symbol(symbol) => write!(f, "{symbol}"),
            TokenType::DoubleSymbol(symbol) => write!(f, "{symbol}{symbol}"),
            TokenType::Newline => write!(f, "\\n"),
            TokenType::End => write!(f, "<end>"),
        }
    }
}
