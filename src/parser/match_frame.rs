use crate::lexer::Lexer;
use crate::parser::ParseError;
use crate::parser::tree::{ParseContext, ParseTree};
use crate::syntax::Highlights;

#[derive(Debug)]
pub(crate) struct MatchFrame<'tree, 'input, C, const H: usize, const P: usize>
where
    C: ParseContext,
{
    tree: &'tree ParseTree<C>,
    child_index: usize,

    lexer: Lexer<'input, P>,
    highlights: Highlights<H>,
    context: C,

    depth: usize,
    error: Option<ParseError>,

    matched: bool,
}

impl<'tree, 'input, C, const H: usize, const P: usize> MatchFrame<'tree, 'input, C, H, P>
where
    C: ParseContext,
{
    pub(crate) fn new(
        tree: &'tree ParseTree<C>,
        lexer: Lexer<'input, P>,
        context: C,
        depth: usize,
    ) -> Self {
        Self {
            tree,
            child_index: 0,
            lexer,
            highlights: Highlights::new(),
            context,
            depth,
            error: None,
            matched: false,
        }
    }

    pub(crate) fn next_child(&mut self) -> Option<&'tree ParseTree<C>> {
        let child = self.tree.children().nth(self.child_index)?;
        self.child_index += 1;

        Some(child)
    }

    pub(crate) fn take_error(&mut self) -> Option<ParseError> {
        self.error.take()
    }

    pub(crate) fn merge_error(&mut self, error: ParseError) {
        error.merge_from_option(&mut self.error);
    }

    pub(crate) fn try_match(&mut self) -> Result<(), ParseError> {
        if self.matched {
            return Ok(());
        }

        self.tree.try_match(
            &mut self.lexer,
            &mut self.highlights,
            &mut self.context,
            self.depth,
        )?;

        self.matched = true;

        Ok(())
    }

    pub(crate) fn is_end(&self) -> bool {
        self.tree.is_end()
    }

    pub(crate) fn into_result(self) -> (Lexer<'input, P>, Highlights<H>, C) {
        (self.lexer, self.highlights, self.context)
    }

    pub(crate) fn child_from(
        parent: &MatchFrame<'tree, 'input, C, H, P>,
        tree: &'tree ParseTree<C>,
    ) -> Self {
        Self {
            tree,
            child_index: 0,

            lexer: parent.lexer.clone(),
            highlights: Highlights::new(),
            context: parent.context.clone(),

            depth: parent.depth + 1,
            error: None,

            matched: false,
        }
    }
}
