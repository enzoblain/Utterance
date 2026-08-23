use smallvec::SmallVec;
use std::iter::Peekable;
use std::str::CharIndices;

use crate::lexer::{LexError, Number, SpecialChar, Symbol, Token, TokenKind, TokenType};
use crate::parser::expectation::StatementKind;
use crate::syntax::Span;

#[derive(Debug, Clone)]
pub struct Lexer<'a, const P: usize> {
    input: &'a str,
    chars: Peekable<CharIndices<'a>>,
    finished: bool,
    peeked: SmallVec<Result<Token<'a>, LexError>, P>,
    comment_symbol: StatementKind,
}

impl<'a, const P: usize> Lexer<'a, P> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.char_indices().peekable(),
            finished: false,
            peeked: SmallVec::new(),
            comment_symbol: StatementKind::DoubleSymbol(Symbol::SpecialChar(SpecialChar::Slash)),
        }
    }

    pub fn set_comment_symbol(mut self, comment_symbol: StatementKind) -> Self {
        self.comment_symbol = comment_symbol;
        self
    }

    pub(crate) fn comment_symbol(&self) -> &StatementKind {
        &self.comment_symbol
    }

    pub fn reset(&mut self) {
        self.chars = self.input.char_indices().peekable();
        self.finished = false;
        self.peeked.clear();
    }

    pub fn update_input(&mut self, input: &'a str) {
        self.input = input;
        self.reset();
    }

    pub fn peek_token(&mut self, n: usize) -> Option<&Result<Token<'a>, LexError>> {
        while self.peeked.len() <= n && !self.finished {
            let token = self.next_token();
            self.peeked.push(token);
        }

        self.peeked.get(n)
    }

    const fn unexpected(&self, position: usize, character: char) -> LexError {
        LexError::UnexpectedCharacter {
            position,
            character,
        }
    }

    fn consume_word(&mut self, start: usize, first: char) -> &'a str {
        let mut end = start + first.len_utf8();

        while let Some(&(index, c)) = self.chars.peek() {
            if c.is_ascii_alphanumeric() {
                self.chars.next();
                end = index + c.len_utf8();
            } else {
                break;
            }
        }

        &self.input[start..end]
    }

    fn consume_number(&mut self, start: usize, first: char) -> (Number, usize) {
        let mut end = start + first.len_utf8();
        let mut has_dot = false;

        while let Some(&(next_index, next_char)) = self.chars.peek() {
            if next_char.is_ascii_digit() {
                self.chars.next();
                end = next_index + next_char.len_utf8();
            } else if !has_dot && next_char == '.' {
                let mut clone = self.chars.clone();
                clone.next();

                if let Some((_, after_dot)) = clone.next()
                    && after_dot.is_ascii_digit()
                {
                    self.chars.next();
                    end = next_index + next_char.len_utf8();
                    has_dot = true;

                    while let Some(&(digit_index, digit_char)) = self.chars.peek() {
                        if digit_char.is_ascii_digit() {
                            self.chars.next();
                            end = digit_index + digit_char.len_utf8();
                        } else {
                            break;
                        }
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        let text = &self.input[start..end];

        let number = if has_dot {
            Number::Float(text.parse().unwrap())
        } else if text.starts_with('-') {
            Number::Integer(text.parse().unwrap())
        } else {
            Number::UnsignedInteger(text.parse().unwrap())
        };

        (number, text.len())
    }

    fn consume_double_symbol(&mut self, symbol: Symbol) -> bool {
        if let Some((_, c)) = self.chars.peek()
            && *c == symbol.as_str().chars().next().unwrap()
        {
            self.chars.next();

            true
        } else {
            false
        }
    }

    fn consume_symbol(&mut self, index: usize, symbol: Symbol) -> Result<Token<'a>, LexError> {
        if self.consume_double_symbol(symbol) {
            self.ensure_no_space_before(index, symbol.as_str().chars().next().unwrap())?;
            self.ensure_space_after(index + 1, symbol.as_str().chars().next().unwrap())?;

            return Ok(Token::new_hardcoded(index, TokenType::DoubleSymbol(symbol)));
        }

        if matches!(symbol, Symbol::Punctuation(_))
            || matches!(
                self.comment_symbol(),
                StatementKind::Symbol(s) if *s == symbol
            )
        {
            self.ensure_no_space_before(index, symbol.as_str().chars().next().unwrap())?;
            self.ensure_space_after(index, symbol.as_str().chars().next().unwrap())?;
        }

        Ok(Token::new_hardcoded(index, TokenType::Symbol(symbol)))
    }

    fn next_token(&mut self) -> Result<Token<'a>, LexError> {
        loop {
            let (index, c) = match self.chars.next() {
                Some(value) => value,
                None => {
                    self.finished = true;
                    return Ok(Token::new_hardcoded(self.input.len(), TokenType::End));
                }
            };

            match c {
                c if c.is_whitespace() => {
                    if c == '\n' {
                        return Ok(Token::new_hardcoded(index, TokenType::Newline));
                    }
                }

                '-' | '+' => {
                    if let Some((_, next)) = self.chars.peek()
                        && next.is_ascii_digit()
                    {
                        let (number, len) = self.consume_number(index, c);
                        return Ok(Token::new_number(index, number, len));
                    }

                    return self.consume_symbol(index, Symbol::SpecialChar(SpecialChar::Minus));
                }

                c if c.is_ascii_digit() => {
                    let (number, len) = self.consume_number(index, c);
                    return Ok(Token::new_number(index, number, len));
                }

                c if c.is_ascii_alphabetic() => {
                    let word = self.consume_word(index, c);
                    return Ok(Token::new(index, word));
                }

                c => {
                    if let Ok(symbol) = Symbol::try_from(c) {
                        return self.consume_symbol(index, symbol);
                    }

                    return Err(self.unexpected(index, c));
                }
            }
        }
    }

    fn ensure_no_space_before(&self, index: usize, character: char) -> Result<(), LexError> {
        if character.is_ascii() {
            return Ok(());
        }

        let has_space_before = index > 0 && self.input.as_bytes()[index - 1].is_ascii_whitespace();
        let preceded_by_special = index > 1
            && !self.input.as_bytes()[index - 2].is_ascii_alphanumeric()
            && !self.input.as_bytes()[index - 2].is_ascii_whitespace();

        if !has_space_before && !preceded_by_special {
            return Err(LexError::NonAsciiPrecededByWhitespace {
                position: index,
                character,
            });
        }

        Ok(())
    }

    fn ensure_space_after(&self, index: usize, character: char) -> Result<(), LexError> {
        let has_whitespace_after = index + character.len_utf8() >= self.input.len()
            || self.input.as_bytes()[index + character.len_utf8()].is_ascii_whitespace();

        if !has_whitespace_after {
            return Err(LexError::NonAsciiNotFollowedByWhitespace {
                position: index,
                character,
            });
        }

        Ok(())
    }

    pub fn finished(&self) -> bool {
        self.finished
    }

    pub fn peek_until_kind(
        &mut self,
        expected: &[TokenKind],
    ) -> Result<(usize, Token<'a>), LexError> {
        let mut count = 0;

        loop {
            let token = match self.peek_token(count).unwrap() {
                Ok(token) => token,
                Err(err) => return Err(err.clone()),
            };

            if expected.contains(&token.token_type().kind()) {
                return Ok((count, token.clone()));
            }

            count += 1;
        }
    }

    pub fn consume_n(&mut self, count: usize) -> Result<Vec<Span>, LexError> {
        let mut spans = Vec::new();

        for _ in 0..count {
            let token = self.next().unwrap()?;
            spans.push(token.span());
        }

        Ok(spans)
    }

    pub fn consume_until_kind(
        &mut self,
        expected: &[TokenKind],
    ) -> Result<(usize, Token<'a>), LexError> {
        let (count, token) = self.peek_until_kind(expected)?;

        self.consume_n(count + 1)?;

        Ok((count, token))
    }
}

impl<'a, const P: usize> Iterator for Lexer<'a, P> {
    type Item = Result<Token<'a>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.peeked.is_empty() {
            return Some(self.peeked.remove(0));
        }

        if self.finished {
            return None;
        }

        Some(self.next_token())
    }
}
