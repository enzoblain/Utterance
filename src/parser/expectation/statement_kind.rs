use crate::lexer::{Lexer, Number, Symbol, TokenType};
use crate::parser::expectation::{
    Expectation, ExpectationError, NumberKind, WordKind, expect_capitalize_word, expect_comment,
    expect_double_symbol, expect_exact_number, expect_exact_word, expect_new_line_or_end,
    expect_noop, expect_number, expect_one_of_words, expect_symbol, expect_word,
};
use crate::syntax::HighlightKind;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum StatementKind {
    Noop,
    CapitalizeWord,
    Word(WordKind),
    OneOfExactWords(&'static [&'static str]),
    Symbol(Symbol),
    DoubleSymbol(Symbol),
    NewLineOrEnd,
    Number(NumberKind),
    ExactNumber(Number),
    Comment,
}

impl StatementKind {
    pub(crate) fn expect<'a, const P: usize>(
        &self,
        lexer: &'a mut Lexer<P>,
    ) -> Result<Expectation<'a>, ExpectationError> {
        let matched = match self {
            StatementKind::Noop => expect_noop(lexer),
            StatementKind::CapitalizeWord => expect_capitalize_word(lexer),
            StatementKind::Word(WordKind::Custom { kind }) => expect_word(lexer, *kind),
            StatementKind::Word(WordKind::Exact(word)) => expect_exact_word(lexer, word),
            StatementKind::OneOfExactWords(words) => expect_one_of_words(lexer, words),
            StatementKind::Symbol(symbol) => expect_symbol(lexer, *symbol),
            StatementKind::DoubleSymbol(symbol) => expect_double_symbol(lexer, *symbol),
            StatementKind::NewLineOrEnd => expect_new_line_or_end(lexer),
            StatementKind::Comment => expect_comment(lexer),
            StatementKind::Number(number) => expect_number(lexer, *number),
            StatementKind::ExactNumber(number) => expect_exact_number(lexer, *number),
        }?;

        let spans = lexer.consume_n(matched.peeked())?;
        Ok(Expectation::new(matched.statement(), spans))
    }
}

impl StatementKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Noop => "noop",
            Self::CapitalizeWord => "capitalize_word",
            Self::Word(word) => match word {
                WordKind::Custom { .. } => "custom word",
                WordKind::Exact(_) => "exact word",
            },
            Self::OneOfExactWords(_) => "one of words",
            Self::Symbol(_) => "symbol",
            Self::DoubleSymbol(_) => "double symbol",
            Self::NewLineOrEnd => "newline_or_end",
            Self::Number(number) => match number {
                NumberKind::UnsignedInteger => "unsigned integer",
                NumberKind::Integer => "integer",
                NumberKind::Float => "float",
            },
            Self::ExactNumber(number) => match number {
                Number::UnsignedInteger(_) => "unsigned integer",
                Number::Integer(_) => "integer",
                Number::Float(_) => "float",
            },
            Self::Comment => "comment",
        }
    }
}

impl<'a> From<&TokenType<'a>> for StatementKind {
    fn from(token: &TokenType) -> Self {
        match token {
            TokenType::Word(_) => StatementKind::Word(WordKind::Custom {
                kind: HighlightKind::Keyword,
            }),
            TokenType::Number(number) => StatementKind::Number((*number).into()),
            TokenType::Symbol(s) => StatementKind::Symbol(*s),
            TokenType::DoubleSymbol(s) => StatementKind::DoubleSymbol(*s),
            TokenType::Newline | TokenType::End => StatementKind::NewLineOrEnd,
        }
    }
}
