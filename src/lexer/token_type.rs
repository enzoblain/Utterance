use crate::lexer::{Number, TokenKind, symbol::Symbol};

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType<'a> {
    Word(&'a str),
    Number(Number),

    Symbol(Symbol),
    DoubleSymbol(Symbol),

    Newline,
    End,
}

impl<'a> TokenType<'a> {
    pub fn kind(&self) -> TokenKind {
        match self {
            TokenType::Word(_) => TokenKind::Word,
            TokenType::Number(_) => TokenKind::Number,

            TokenType::Symbol(_) => TokenKind::Symbol,
            TokenType::DoubleSymbol(_) => TokenKind::DoubleSymbol,

            TokenType::Newline => TokenKind::Newline,
            TokenType::End => TokenKind::End,
        }
    }
}
