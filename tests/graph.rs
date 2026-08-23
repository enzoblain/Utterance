use std::rc::Rc;

use utterance::empty_children;
use utterance::lexer::{Lexer, Punctuation, SpecialChar, Symbol};
use utterance::parser::expectation::{NumberKind, StatementKind};
use utterance::parser::graph::GraphBuilder;
use utterance::parser::tree::{ParseContext, ParseTree, ParseTreeFactory, noop_fn};
use utterance::parser::{CustomParseError, Parser};

#[derive(Clone, Default, Debug)]
struct NoopContext;

impl ParseContext for NoopContext {
    type Error = CustomParseError;
}

#[test]
pub(crate) fn cyclic_base_tree() {
    let mut graph = GraphBuilder::<NoopContext>::new();
    let tree_factory = ParseTreeFactory::<NoopContext>::new();

    let end_tree = ParseTree::new(
        Rc::clone(&tree_factory),
        StatementKind::NewLineOrEnd,
        empty_children!(),
        noop_fn,
    );

    let right_bracket_tree = ParseTree::new(
        Rc::clone(&tree_factory),
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::RightBracket)),
        [end_tree],
        noop_fn,
    );

    let right_bracket_node = graph.add_tree(&right_bracket_tree);

    let number_node = graph.add_node(StatementKind::Number(NumberKind::Float), noop_fn);

    let comma_node = graph.add_node(
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::Comma)),
        noop_fn,
    );

    graph.add_child(comma_node, number_node);
    graph.add_children(number_node, [comma_node, right_bracket_node]);

    let repetition_tree = graph.build_from(Rc::clone(&tree_factory), number_node);

    let mut tree = ParseTree::new(
        Rc::clone(&tree_factory),
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::LeftBracket)),
        [right_bracket_tree],
        noop_fn,
    );

    tree.push_child(repetition_tree);

    let success = [
        "[]",
        "[1]",
        "[-1.55]",
        "[1, 1.0, -5]",
        "[1, 2, 3, 4, 5]",
        "[1, 1, 1, 1, 1, 1]",
    ];

    let deep = format!(
        "[{}]",
        (0..10_000).map(|_| "1").collect::<Vec<_>>().join(", ")
    );

    let failures = [
        "[", "]", "15]", "[15, ]", "[15],", "[,15]", "[1,,2]", "[1 2]",
    ];

    let lexer = Lexer::<1>::new("");
    let mut parser = Parser::<NoopContext, 1, 1>::new(lexer, tree);

    for input in success {
        parser.update_input(input);

        let res = parser.parse();

        assert!(
            res.is_ok(),
            "expected success for {:?}, got: {:?}",
            input,
            res.unwrap_err()
        );
    }

    for input in failures {
        parser.update_input(input);

        let res = parser.parse();

        assert!(
            res.is_err(),
            "expected failure for {:?}, but parsing succeeded",
            input
        );
    }

    parser.update_input(&deep);

    let res = parser.parse();

    assert!(
        res.is_ok(),
        "expected success for deep cyclic parse, got: {:?}",
        res.unwrap_err()
    );
}
