use std::ptr::fn_addr_eq;
use std::rc::Rc;

use crate::empty_children;
use crate::lexer::Lexer;
use crate::parser::expectation::{Expectation, ExpectationError, Statement, StatementKind};
use crate::parser::tree::factory::ParseTreeFactory;
use crate::parser::tree::{Child, ParseContext, TreeFn, noop_fn};
use crate::parser::{CustomParseError, ParseError};
use crate::syntax::{Highlight, HighlightKind, Highlights};

#[derive(Debug, Clone)]
pub struct ParseTree<C>
where
    C: ParseContext,
{
    tree_factory: Rc<ParseTreeFactory<C>>,

    statement_expectation: StatementKind,
    function: TreeFn<C>,

    children: Vec<Child<C>>,
}

impl<C> ParseTree<C>
where
    C: ParseContext,
{
    pub fn new<I, T>(
        tree_factory: Rc<ParseTreeFactory<C>>,
        statement_expectation: StatementKind,
        children: I,
        function: TreeFn<C>,
    ) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Child<C>>,
    {
        Self {
            tree_factory,
            statement_expectation,
            children: children.into_iter().map(Into::into).collect(),
            function,
        }
    }

    pub fn tree_factory(&self) -> &ParseTreeFactory<C> {
        &self.tree_factory
    }

    fn check<'a, const P: usize>(
        &self,
        lexer: &'a mut Lexer<P>,
    ) -> Result<Expectation<'a>, ExpectationError> {
        self.statement_expectation.expect(lexer)
    }

    fn do_function(
        &self,
        context: &mut C,
        statement: &Statement,
        depth: usize,
    ) -> Result<(), ParseError> {
        (self.function)(context, statement).map_err(|err| {
            ParseError::custom(depth, Into::<CustomParseError>::into(err).to_string())
        })
    }

    pub(crate) fn try_match<const H: usize, const P: usize>(
        &self,
        lexer: &mut Lexer<P>,
        highlights: &mut Highlights<H>,
        context: &mut C,
        depth: usize,
    ) -> Result<(), ParseError> {
        let expectation = self
            .check(lexer)
            .map_err(|err| err.into_parse_error(depth))?;

        if let Err(err) = self.do_function(context, expectation.statement(), depth) {
            highlights.extend(
                expectation
                    .span()
                    .into_iter()
                    .map(|span| Highlight::new(HighlightKind::Error, span)),
            );

            return Err(err);
        }

        let highlight_kind = expectation.statement().highlight_kind();
        highlights.extend(
            expectation
                .span()
                .into_iter()
                .map(|span| Highlight::new(highlight_kind, span)),
        );

        Ok(())
    }

    pub fn expectation(&self) -> &StatementKind {
        &self.statement_expectation
    }

    pub fn function(&self) -> &TreeFn<C> {
        &self.function
    }

    pub(crate) fn is_end(&self) -> bool {
        self.children.is_empty()
    }

    pub(crate) fn is_owned_end(&mut self) -> bool {
        self.children_mut().next().is_none()
    }

    pub fn push_child<T>(&mut self, child: T)
    where
        T: Into<Child<C>>,
    {
        self.children.push(child.into());
    }

    pub fn push_child_to_leaves<T>(&mut self, child: T)
    where
        T: Into<Child<C>>,
    {
        let child = child.into().into_ref();

        if self.is_end() {
            self.children.push(child);

            return;
        }

        self.children_mut()
            .for_each(|c| c.push_child_to_leaves(child.clone()));
    }

    pub fn push_child_to_owned_leaves<T>(&mut self, child: T)
    where
        T: Into<Child<C>>,
    {
        let child = child.into().into_ref();

        if self.is_owned_end() {
            self.children.push(child);
            return;
        }

        self.children_mut()
            .for_each(|c| c.push_child_to_owned_leaves(child.clone()));
    }

    pub fn push_children<I, T>(&mut self, children: I)
    where
        I: IntoIterator<Item = T>,
        T: Into<Child<C>>,
    {
        self.children.extend(children.into_iter().map(Into::into));
    }

    pub(crate) fn children(&self) -> impl Iterator<Item = &ParseTree<C>> {
        self.children.iter().map(|child| child.as_tree())
    }

    pub(crate) fn children_mut(&mut self) -> impl Iterator<Item = &mut ParseTree<C>> {
        self.children
            .iter_mut()
            .filter_map(|child| child.as_tree_mut())
    }

    fn same_kind(&self, other: &Self) -> bool {
        self.statement_expectation == other.statement_expectation
            && fn_addr_eq(self.function, other.function)
    }

    pub fn merge(&mut self, other: Self) {
        if !self.same_kind(&other) {
            *self = ParseTree::noop(
                self.tree_factory.clone(),
                vec![
                    std::mem::replace(
                        self,
                        ParseTree::noop(self.tree_factory.clone(), empty_children!()),
                    ),
                    other,
                ],
            );
            return;
        }

        for other_child in other.children {
            let other_tree = match other_child.into_owned() {
                Ok(tree) => tree,
                Err(child) => {
                    self.children.push(child);
                    continue;
                }
            };

            if let Some(self_tree) = self.children.iter_mut().find_map(|child| {
                child
                    .as_tree_mut()
                    .filter(|tree| tree.same_kind(&other_tree))
            }) {
                self_tree.merge(other_tree);
            } else {
                self.children.push(Child::Contains(other_tree));
            }
        }
    }

    pub fn root<I, T>(tree_factory: Rc<ParseTreeFactory<C>>, children: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Child<C>>,
    {
        Self::noop(
            tree_factory.clone(),
            children.into_iter().map(Into::into).chain(
                [tree_factory.comments(), tree_factory.line_terminator()]
                    .into_iter()
                    .map(Into::into),
            ),
        )
    }

    pub fn noop<I, T>(tree_factory: Rc<ParseTreeFactory<C>>, children: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Child<C>>,
    {
        ParseTree::new(tree_factory, StatementKind::Noop, children, noop_fn)
    }

    pub fn add_end_of_line_fn(&mut self) {
        if self.is_end() {
            self.push_child(self.tree_factory.statement_terminator());
            return;
        }

        self.children_mut().for_each(ParseTree::add_end_of_line_fn);
    }

    pub fn into_line(mut self) -> Self {
        self.add_end_of_line_fn();

        Self::new(
            self.tree_factory.clone(),
            StatementKind::CapitalizeWord,
            [self],
            noop_fn,
        )
    }
}

#[macro_export]
macro_rules! expect_words {
    ($factory:expr; $($word:expr),+; $children:expr; $callback:expr $(,)?) => {{
        expect_words!(@build $factory, $callback, $children; $($word),+)
    }};

    (@build $factory:expr, $callback:expr, $children:expr; $last:expr) => {
        $crate::parser::tree::ParseTree::new(
            $factory,
            $crate::parser::expectation::StatementKind::Word(
                $crate::parser::expectation::WordKind::Exact($last),
            ),
            $children,
            $callback,
        )
    };

    (@build $factory:expr, $callback:expr, $children:expr; $head:expr, $($tail:expr),+) => {
        $crate::parser::tree::ParseTree::new(
            $factory.clone(),
            $crate::parser::expectation::StatementKind::Word(
                $crate::parser::expectation::WordKind::Exact($head),
            ),
            [expect_words!(@build $factory, $callback, $children; $($tail),+)],
            $crate::parser::tree::noop_fn,
        )
    };
}
#[macro_export]
macro_rules! merge_trees {
    ($tree:expr $(,)?) => {
        $tree
    };

    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut tree = $first;
        $(
            tree.merge($rest);
        )+
        tree
    }};
}

pub use expect_words;
pub use merge_trees;
