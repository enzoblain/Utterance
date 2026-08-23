use std::cell::OnceCell;
use std::rc::Rc;

use crate::empty_children;
use crate::lexer::{Punctuation, Symbol};
use crate::parser::expectation::StatementKind;
use crate::parser::tree::{ParseContext, ParseTree, noop_fn};

#[derive(Debug)]
pub struct ParseTreeFactory<C>
where
    C: ParseContext,
{
    comments: OnceCell<Rc<ParseTree<C>>>,
    line_terminator: OnceCell<Rc<ParseTree<C>>>,
    statement_terminator: OnceCell<Rc<ParseTree<C>>>,
}

impl<C> ParseTreeFactory<C>
where
    C: ParseContext,
{
    pub fn new() -> Rc<Self> {
        let factory = Rc::new(Self {
            comments: OnceCell::new(),
            line_terminator: OnceCell::new(),
            statement_terminator: OnceCell::new(),
        });

        let line_terminator = Rc::new(ParseTree::new(
            Rc::clone(&factory),
            StatementKind::NewLineOrEnd,
            empty_children!(),
            noop_fn,
        ));

        let _ = factory.line_terminator.set(Rc::clone(&line_terminator));

        let comments = Rc::new(ParseTree::new(
            Rc::clone(&factory),
            StatementKind::Comment,
            [Rc::clone(&line_terminator)],
            noop_fn,
        ));

        let _ = factory.comments.set(Rc::clone(&comments));

        let statement_terminator = Rc::new(ParseTree::new(
            Rc::clone(&factory),
            StatementKind::Symbol(Symbol::Punctuation(Punctuation::Point)),
            [Rc::clone(&comments), Rc::clone(&line_terminator)],
            noop_fn,
        ));

        let _ = factory
            .statement_terminator
            .set(Rc::clone(&statement_terminator));

        factory
    }

    pub fn comments(&self) -> Rc<ParseTree<C>> {
        Rc::clone(self.comments.get().unwrap())
    }

    pub fn line_terminator(&self) -> Rc<ParseTree<C>> {
        Rc::clone(self.line_terminator.get().unwrap())
    }

    pub fn statement_terminator(&self) -> Rc<ParseTree<C>> {
        Rc::clone(self.statement_terminator.get().unwrap())
    }
}
