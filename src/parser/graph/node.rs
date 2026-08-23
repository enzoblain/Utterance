use std::collections::HashSet;

use crate::parser::expectation::StatementKind;
use crate::parser::tree::{ParseContext, TreeFn};

pub type GraphNodeId = usize;

#[derive(Debug)]
pub struct GraphNode<C>
where
    C: ParseContext,
{
    statement_expectation: StatementKind,
    function: TreeFn<C>,
    children: HashSet<GraphNodeId>,
}

impl<C> GraphNode<C>
where
    C: ParseContext,
{
    pub(crate) fn new(statement_expectation: StatementKind, function: TreeFn<C>) -> Self {
        Self {
            statement_expectation,
            function,
            children: HashSet::new(),
        }
    }

    pub(crate) fn push_child(&mut self, child: GraphNodeId) {
        self.children.insert(child);
    }

    pub(crate) fn children(&self) -> impl Iterator<Item = &GraphNodeId> {
        self.children.iter()
    }

    pub(crate) fn statement_expectation(&self) -> &StatementKind {
        &self.statement_expectation
    }

    pub(crate) fn function(&self) -> TreeFn<C> {
        self.function
    }
}
