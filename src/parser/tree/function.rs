use crate::parser::expectation::Statement;
use crate::parser::tree::ParseContext;

pub fn noop_fn<C>(_: &mut C, _: &Statement) -> Result<(), C::Error>
where
    C: ParseContext,
{
    Ok(())
}

pub type TreeFn<C> = fn(&mut C, &Statement) -> Result<(), <C as ParseContext>::Error>;

#[macro_export]
macro_rules! tree_fn {
    (Noop($context:ident) => $body:block) => {
        |$context, statement| match statement {
            $crate::parser::expectation::Statement::Noop => $body,
            _ => unreachable!(),
        }
    };

    (Comment($context:ident) => $body:block) => {
        |$context, statement| match statement {
            $crate::parser::expectation::Statement::Comment => $body,
            _ => unreachable!(),
        }
    };

    (Word($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Word(value) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (ExactWord($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Word($crate::lexer::Word::Exact(value)) => {
                    value
                }
                _ => unreachable!(),
            };

            $body
        }
    };

    (Symbol($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Symbol(value) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (Punctuation($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Symbol(
                    $crate::lexer::Symbol::Punctuation(value),
                ) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (SpecialChar($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Symbol(
                    $crate::lexer::Symbol::SpecialChar(value),
                ) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (Number($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Number(value) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (UnsignedInteger($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Number(
                    $crate::lexer::Number::UnsignedInteger(value),
                ) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (Integer($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Number($crate::lexer::Number::Integer(
                    value,
                )) => value,
                _ => unreachable!(),
            };

            $body
        }
    };

    (Float($context:ident, $value:ident) => $body:block) => {
        |$context, statement| {
            let $value = match statement {
                $crate::parser::expectation::Statement::Number($crate::lexer::Number::Float(
                    value,
                )) => value,
                _ => unreachable!(),
            };

            $body
        }
    };
}

#[macro_export]
macro_rules! setter_fn {
    ($($call:tt)+) => {
        |context, _| {
            context.$($call)+
        }
    };
}

pub use setter_fn;
pub use tree_fn;
