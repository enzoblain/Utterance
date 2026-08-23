use std::rc::Rc;

use crate::parser::tree::{ParseContext, ParseTree};

#[derive(Debug, Clone)]
pub enum Child<C>
where
    C: ParseContext,
{
    Contains(ParseTree<C>),
    ByRef(Rc<ParseTree<C>>),
}

impl<C> Child<C>
where
    C: ParseContext,
{
    pub fn as_tree(&self) -> &ParseTree<C> {
        match self {
            Self::Contains(tree) => tree,
            Self::ByRef(ptr) => ptr,
        }
    }

    pub fn as_tree_mut(&mut self) -> Option<&mut ParseTree<C>> {
        match self {
            Self::Contains(tree) => Some(tree),
            Self::ByRef(_) => None,
        }
    }

    pub fn into_owned(self) -> Result<ParseTree<C>, Self> {
        match self {
            Self::Contains(tree) => Ok(tree),
            other => Err(other),
        }
    }

    pub(crate) fn into_ref(self) -> Self {
        match self {
            Child::Contains(tree) => Child::ByRef(Rc::new(tree)),
            child => child,
        }
    }
}

impl<C> From<ParseTree<C>> for Child<C>
where
    C: ParseContext,
{
    fn from(tree: ParseTree<C>) -> Self {
        Child::Contains(tree)
    }
}

impl<C> From<Rc<ParseTree<C>>> for Child<C>
where
    C: ParseContext,
{
    fn from(tree: Rc<ParseTree<C>>) -> Self {
        Child::ByRef(tree)
    }
}

#[macro_export]
macro_rules! empty_children {
    () => {
        ::std::iter::empty::<$crate::parser::tree::Child<_>>()
    };
}

pub use empty_children;
