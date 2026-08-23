use crate::parser::expectation::Statement;
use crate::syntax::Span;

pub(crate) struct Expectation<'a> {
    spans: Vec<Span>,
    statement: Statement<'a>,
}

impl<'a> Expectation<'a> {
    pub(crate) fn new(statement: Statement<'a>, spans: Vec<Span>) -> Self {
        Self { spans, statement }
    }

    pub(crate) fn span(self) -> Vec<Span> {
        self.spans
    }

    pub(crate) fn statement(&self) -> &Statement<'a> {
        &self.statement
    }
}
