use std::fmt;

use crate::parser::expectation::WordKind;
use crate::syntax::HighlightKind;

#[derive(Debug, Copy, Clone)]
pub enum Word<'word> {
    Custom {
        word: &'word str,
        kind: HighlightKind,
    },
    Exact(&'static str),
}

impl<'word> Word<'word> {
    pub(crate) fn highligh_kind(&self) -> HighlightKind {
        match self {
            Self::Custom { word: _, kind } => *kind,
            Self::Exact(_) => HighlightKind::None,
        }
    }
}

impl<'word> Eq for Word<'word> {}

impl<'word> PartialEq for Word<'word> {
    fn eq(&self, other: &Self) -> bool {
        let lhs = match self {
            Word::Custom { word, .. } => *word,
            Word::Exact(word) => *word,
        };

        let rhs = match other {
            Word::Custom { word, .. } => *word,
            Word::Exact(word) => *word,
        };

        lhs == rhs
    }
}

impl<'word> fmt::Display for Word<'word> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom { word, .. } => write!(f, "{word}"),
            Self::Exact(word) => write!(f, "{word}"),
        }
    }
}

impl<'word> From<Word<'word>> for WordKind {
    fn from(word: Word) -> Self {
        match word {
            Word::Custom { word: _, kind } => WordKind::Custom { kind },
            Word::Exact(w) => WordKind::Exact(w),
        }
    }
}
