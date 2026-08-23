use crate::lexer::Lexer;
use crate::parser::tree::{ParseContext, ParseTree};
use crate::parser::{MatchFrame, ParseError};
use crate::syntax::{Highlight, HighlightKind, Highlights};

#[derive(Debug)]
pub struct Parser<'input, C, const H: usize, const P: usize>
where
    C: ParseContext,
{
    lexer: Lexer<'input, P>,
    base_tree: ParseTree<C>,
    context: C,
    highlights: Highlights<H>,
}

impl<'input, C, const H: usize, const P: usize> Parser<'input, C, H, P>
where
    C: ParseContext,
{
    pub fn new(lexer: Lexer<'input, P>, base_tree: ParseTree<C>) -> Parser<'input, C, H, P> {
        Parser {
            lexer,
            base_tree,
            context: C::default(),
            highlights: Highlights::new(),
        }
    }

    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.reset();

        while !self.lexer.finished() {
            let mut lexer = self.lexer.clone();
            let mut highlights = Highlights::new();
            let mut context = self.context.clone();

            Self::match_tree(
                &self.base_tree,
                &mut lexer,
                &mut highlights,
                &mut context,
                0,
            )?;

            self.lexer = lexer;
            self.highlights.extend(
                highlights
                    .iter()
                    .filter(|h| h.kind() != &HighlightKind::None),
            );
            self.context = context;
        }

        Ok(())
    }

    fn match_tree(
        tree: &ParseTree<C>,
        lexer: &mut Lexer<'input, P>,
        highlights: &mut Highlights<H>,
        context: &mut C,
        depth: usize,
    ) -> Result<(), ParseError> {
        let mut stack: Vec<MatchFrame<'_, 'input, C, H, P>> =
            vec![MatchFrame::new(tree, lexer.clone(), context.clone(), depth)];

        while let Some(frame) = stack.last_mut() {
            if let Err(err) = frame.try_match() {
                stack.pop();

                if let Some(parent) = stack.last_mut() {
                    parent.merge_error(err);
                    continue;
                }

                return Err(err);
            }

            if frame.is_end() {
                let frame = stack.pop().unwrap();
                let (new_lexer, new_highlights, new_context) = frame.into_result();

                *lexer = new_lexer;
                *context = new_context;
                highlights.extend(new_highlights);

                return Ok(());
            }

            match frame.next_child() {
                Some(child) => {
                    let child_frame = MatchFrame::child_from(frame, child);
                    stack.push(child_frame);
                }

                None => {
                    let error = frame.take_error().expect("missing parse error");
                    stack.pop();

                    if let Some(parent) = stack.last_mut() {
                        parent.merge_error(error);
                    } else {
                        return Err(error);
                    }
                }
            }
        }

        unreachable!()
    }

    pub fn reset(&mut self) {
        self.lexer.reset();
        self.context = C::default();
        self.highlights.clear();
    }

    pub fn update_input(&mut self, input: &'input str) {
        self.lexer.update_input(input);
        self.reset();
    }

    pub fn context(&self) -> &C {
        &self.context
    }

    pub fn highlights(&self) -> &[Highlight] {
        &self.highlights
    }
}
