use utterance::empty_children;
use utterance::lexer::{Lexer, SpecialChar};
use utterance::lexer::{Number, Punctuation, Symbol};
use utterance::parser::expectation::{NumberKind, StatementKind, WordKind};
use utterance::parser::tree::{ParseContext, ParseTree, ParseTreeFactory, noop_fn};
use utterance::parser::{CustomParseError, Parser};
use utterance::syntax::HighlightKind;

impl ParseContext for NoopContext {
    type Error = CustomParseError;
}

macro_rules! assert_parse {
    ($parser:expr, $input:expr) => {{
        $parser.update_input($input);

        match $parser.parse() {
            Ok(_) => {}
            Err(err) => {
                panic!("\nParse failed\nInput: {:?}\nError: {:#?}\n", $input, err);
            }
        }
    }};
}

macro_rules! assert_parse_err {
    ($parser:expr, $input:expr) => {{
        $parser.update_input($input);

        match $parser.parse() {
            Ok(_) => {
                panic!(
                    "\nExpected parse failure but succeeded\nInput: {:?}\n",
                    $input
                );
            }
            Err(err) => {
                println!("\nExpected error\nInput: {:?}\nError: {:#?}\n", $input, err);
            }
        }
    }};
}

macro_rules! end_tree {
    ($factory:expr) => {{
        ParseTree::new(
            $factory.clone(),
            StatementKind::NewLineOrEnd,
            empty_children!(),
            noop_fn,
        )
    }};
}

macro_rules! statement_tree {
    ($factory:expr, $statement:expr) => {{
        ParseTree::new(
            $factory.clone(),
            $statement,
            vec![end_tree!($factory)],
            noop_fn,
        )
    }};
}

macro_rules! parser_for {
    ($tree:expr) => {{ Parser::<'static, NoopContext, 1, 1>::new(Lexer::<1>::new(""), $tree) }};
}

#[derive(Clone, Default, Debug)]
struct NoopContext;

#[test]
fn test_noop_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let consume = statement_tree!(
        factory.clone(),
        StatementKind::Word(WordKind::Custom {
            kind: HighlightKind::Keyword
        })
    );
    let tree = ParseTree::new(factory, StatementKind::Noop, vec![consume], noop_fn);

    let mut parser = parser_for!(tree);
    assert_parse!(parser, "a");
}

#[test]
fn test_capitalize_word_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let consume = statement_tree!(
        factory.clone(),
        StatementKind::Word(WordKind::Custom {
            kind: HighlightKind::Keyword
        })
    );
    let tree = ParseTree::new(
        factory,
        StatementKind::CapitalizeWord,
        vec![consume],
        noop_fn,
    );

    let mut parser = parser_for!(tree);
    for input in ["Hello", "World", "Rust"] {
        assert_parse!(parser, input);
    }
}

#[test]
fn test_word_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = statement_tree!(
        factory,
        StatementKind::Word(WordKind::Custom {
            kind: HighlightKind::Keyword
        })
    );

    let mut parser = parser_for!(tree);
    for input in ["hello", "abc123", "rust"] {
        assert_parse!(parser, input);
    }
}

#[test]
fn test_exact_word_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = statement_tree!(factory, StatementKind::Word(WordKind::Exact("hello")));

    let mut parser = parser_for!(tree);
    assert_parse!(parser, "hello");
    assert_parse_err!(parser, "world");
}

#[test]
fn test_symbol_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = statement_tree!(
        factory,
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::Comma))
    );

    let mut parser = parser_for!(tree);
    assert_parse!(parser, ",");
    assert_parse_err!(parser, ".");
}

#[test]
fn test_double_symbol_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = statement_tree!(
        factory,
        StatementKind::DoubleSymbol(Symbol::Punctuation(Punctuation::Comma))
    );

    let mut parser = parser_for!(tree);
    assert_parse!(parser, ",,");
    assert_parse_err!(parser, ",");
}

#[test]
fn test_new_line_or_end_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = ParseTree::new(
        factory,
        StatementKind::NewLineOrEnd,
        empty_children!(),
        noop_fn,
    );

    let mut parser = parser_for!(tree);
    assert_parse!(parser, "");
    assert_parse!(parser, "\n");
}

#[test]
fn test_number_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();
    let tree = statement_tree!(factory, StatementKind::Number(NumberKind::Float));

    let mut parser = parser_for!(tree);
    for input in ["1", "1.5", "-5", "-3.14"] {
        assert_parse!(parser, input);
    }
}

#[test]
fn test_exact_number_statement() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    let tree = statement_tree!(factory, StatementKind::ExactNumber(Number::Integer(42)));

    let mut parser = parser_for!(tree);
    assert_parse!(parser, "42");
    assert_parse_err!(parser, "43");
}

#[test]
fn test_comment_statement() {
    let comments = [
        StatementKind::DoubleSymbol(Symbol::Punctuation(Punctuation::SemiColon)),
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::SemiColon)),
        StatementKind::DoubleSymbol(Symbol::Punctuation(Punctuation::Comma)),
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::Comma)),
        StatementKind::DoubleSymbol(Symbol::SpecialChar(SpecialChar::Slash)),
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::Slash)),
    ];

    for comment_type in comments {
        let factory = ParseTreeFactory::<NoopContext>::new();

        let (inputs, invalid_inputs) = match comment_type {
            StatementKind::DoubleSymbol(symbol) => (
                vec![
                    format!("{}{} hello", symbol.as_str(), symbol.as_str()),
                    format!("{}{}", symbol.as_str(), symbol.as_str()),
                ],
                vec![format!("{}{}hello", symbol.as_str(), symbol.as_str())],
            ),

            StatementKind::Symbol(symbol) => (
                vec![
                    format!("{} hello", symbol.as_str()),
                    symbol.as_str().to_string(),
                ],
                vec![format!("{}hello", symbol.as_str())],
            ),

            _ => unreachable!(),
        };

        let tree = statement_tree!(factory.clone(), StatementKind::Comment);

        let mut parser = Parser::<NoopContext, 1, 1>::new(
            Lexer::<1>::new("").set_comment_symbol(comment_type),
            tree,
        );

        for input in &inputs {
            assert_parse!(parser, input);
        }

        for input in &invalid_inputs {
            assert_parse_err!(parser, input);
        }
    }
}
