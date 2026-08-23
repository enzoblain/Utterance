use crate::lexer::{Number, Symbol, Word};
use crate::syntax::HighlightKind;

pub enum Statement<'word> {
    Noop,
    Word(Word<'word>),
    Symbol(Symbol),
    DoubleSymbol(Symbol),
    NewLineOrEnd,
    Number(Number),
    Comment,
}

impl<'a> Statement<'a> {
    pub(crate) fn highlight_kind(&self) -> HighlightKind {
        match self {
            Statement::Word(w) => w.highligh_kind(),
            Statement::Number(_) => HighlightKind::Number,
            Statement::Comment => HighlightKind::Comment,
            Statement::Symbol(_)
            | Statement::DoubleSymbol(_)
            | Statement::Noop
            | Statement::NewLineOrEnd => HighlightKind::None,
        }
    }
}
