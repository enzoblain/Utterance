use smallvec::SmallVec;

use super::Span;

pub(crate) type Highlights<const H: usize> = SmallVec<Highlight, H>;

#[derive(Clone, Debug)]
pub struct Highlight {
    kind: HighlightKind,
    span: Span,
}

impl Highlight {
    pub(crate) fn new(kind: HighlightKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn kind(&self) -> &HighlightKind {
        &self.kind
    }

    pub fn span(&self) -> &Span {
        &self.span
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HighlightKind {
    None,
    Keyword,
    Function,
    Variable,
    Number,
    String,
    Comment,
    Error,
}
