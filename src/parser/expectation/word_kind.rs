use crate::syntax::HighlightKind;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum WordKind {
    Custom { kind: HighlightKind },
    Exact(&'static str),
}
