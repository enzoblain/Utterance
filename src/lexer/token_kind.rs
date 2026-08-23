#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Word,
    Number,

    Symbol,
    DoubleSymbol,

    Newline,
    End,
}
