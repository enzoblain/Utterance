use crate::parser::expectation::Statement;

pub(crate) struct Match<'a> {
    peeked: usize,
    statement: Statement<'a>,
}

impl<'a> Match<'a> {
    pub(crate) fn new(peeked: usize, statement: Statement<'a>) -> Self {
        Self { peeked, statement }
    }

    pub(crate) fn peeked(&self) -> usize {
        self.peeked
    }

    pub(crate) fn statement(self) -> Statement<'a> {
        self.statement
    }
}
