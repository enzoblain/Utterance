use std::rc::Rc;

use utterance::empty_children;
use utterance::expect_words;
use utterance::lexer::Lexer;
use utterance::lexer::Number;
use utterance::lexer::Punctuation;
use utterance::lexer::SpecialChar;
use utterance::lexer::Symbol;
use utterance::parser::CustomParseError;
use utterance::parser::Parser;
use utterance::parser::expectation::{NumberKind, Statement, StatementKind};
use utterance::parser::graph::GraphBuilder;
use utterance::parser::tree::Child;
use utterance::parser::tree::ParseContext;
use utterance::parser::tree::{ParseTree, ParseTreeFactory, noop_fn};

#[derive(Clone, Debug)]
struct TestContext {
    n1: Option<f64>,
    result: Option<f64>,
    operation: Operation,
}

impl Default for TestContext {
    fn default() -> Self {
        Self {
            n1: None,
            result: None,
            operation: Operation::Sum,
        }
    }
}

impl ParseContext for TestContext {
    type Error = CustomParseError;
}

impl ParseContext for TestSumContext {
    type Error = CustomParseError;
}

#[derive(Clone, Debug)]
enum Operation {
    Sum,
    Difference,
}

fn set_sum_operation(context: &mut TestContext, _: &Statement) -> Result<(), CustomParseError> {
    context.operation = Operation::Sum;

    Ok(())
}

fn set_difference_operation(
    context: &mut TestContext,
    _: &Statement,
) -> Result<(), CustomParseError> {
    context.operation = Operation::Difference;

    Ok(())
}

fn store_first(context: &mut TestContext, statement: &Statement) -> Result<(), CustomParseError> {
    let Statement::Number(Number::Float(value)) = statement else {
        unreachable!()
    };

    context.n1 = Some(*value);

    Ok(())
}

fn compute_result(
    context: &mut TestContext,
    statement: &Statement,
) -> Result<(), CustomParseError> {
    let Statement::Number(Number::Float(value)) = statement else {
        unreachable!()
    };

    let first = context
        .n1
        .ok_or_else(|| CustomParseError::new("missing first number"))?;

    context.result = Some(match context.operation {
        Operation::Sum => first + *value,
        Operation::Difference => first - *value,
    });

    Ok(())
}

fn check_result(context: &mut TestContext, statement: &Statement) -> Result<(), CustomParseError> {
    let Statement::Number(Number::Float(actual)) = statement else {
        unreachable!()
    };

    let expected = context
        .result
        .ok_or_else(|| CustomParseError::new("missing result"))?;

    if *actual == expected {
        Ok(())
    } else {
        Err(CustomParseError::new(format!(
            "expected {}, got {}",
            expected, actual
        )))
    }
}

fn result_tree(factory: Rc<ParseTreeFactory<TestContext>>) -> ParseTree<TestContext> {
    expect_words!(
        factory.clone();
        "is";
        vec![
            ParseTree::new(
                factory.clone(),
                StatementKind::Number(NumberKind::Float),
                empty_children!(),
                check_result,
            ),
        ];
        noop_fn
    )
}

#[test]
fn test_number_addition_with_context() {
    let factory = ParseTreeFactory::<TestContext>::new();

    let tree = expect_words!(
        factory.clone();
        "The";
        vec![
            expect_words!(
                factory.clone();
                "sum",
                "of";
                vec![
                    ParseTree::new(
                        factory.clone(),
                        StatementKind::Number(NumberKind::Float),
                        vec![
                            expect_words!(
                                factory.clone();
                                "and";
                                vec![
                                    ParseTree::new(
                                        factory.clone(),
                                        StatementKind::Number(NumberKind::Float),
                                        vec![
                                            result_tree(factory.clone()),
                                        ],
                                        compute_result,
                                    ),
                                ];
                                noop_fn
                            ),
                        ],
                        store_first,
                    ),
                ];
                set_sum_operation
            ),
            expect_words!(
                factory.clone();
                "difference",
                "between";
                vec![
                    ParseTree::new(
                        factory.clone(),
                        StatementKind::Number(NumberKind::Float),
                        vec![
                            expect_words!(
                                factory.clone();
                                "and";
                                vec![
                                    ParseTree::new(
                                        factory.clone(),
                                        StatementKind::Number(NumberKind::Float),
                                        vec![
                                            result_tree(factory.clone()),
                                        ],
                                        compute_result,
                                    ),
                                ];
                                noop_fn
                            ),
                        ],
                        store_first,
                    ),
                ];
                set_difference_operation
            ),
        ];
        noop_fn
    )
    .into_line();

    let inputs = [
        "The sum of 2.5 and 3.5 is 6.0.",
        "The difference between 5.0 and 2.0 is 3.0.",
    ];

    let lexer = Lexer::<1>::new("");
    let mut parser = Parser::<TestContext, 1, 1>::new(lexer, tree);

    for input in inputs {
        parser.update_input(input);
        let res = parser.parse();

        assert!(
            res.is_ok(),
            "expected success for '{}', got: {:?}",
            input,
            res.unwrap_err()
        );
    }
}

#[derive(Clone, Default, Debug)]
struct TestSumContext {
    sum: f64,
}

fn add_number(context: &mut TestSumContext, statement: &Statement) -> Result<(), CustomParseError> {
    let Statement::Number(Number::Float(value)) = statement else {
        unreachable!()
    };

    context.sum += *value;

    Ok(())
}

fn check_sum(context: &mut TestSumContext, statement: &Statement) -> Result<(), CustomParseError> {
    let Statement::Number(Number::Float(value)) = statement else {
        unreachable!()
    };

    if context.sum == *value {
        Ok(())
    } else {
        Err(CustomParseError::new(format!(
            "expected {}, got {}",
            context.sum, value
        )))
    }
}

fn end_tree(factory: Rc<ParseTreeFactory<TestSumContext>>) -> ParseTree<TestSumContext> {
    let mut tree = expect_words!(
        factory.clone();
        "is";
        [
            ParseTree::new(
                factory.clone(),
                StatementKind::Number(NumberKind::Float),
                [] as [Child<TestSumContext>; 0],
                check_sum,
            )
        ];
        noop_fn
    );
    tree.add_end_of_line_fn();
    tree
}

#[test]
fn test_array_sum_with_context() {
    let factory = ParseTreeFactory::<TestSumContext>::new();

    let mut graph = GraphBuilder::<TestSumContext>::new();

    let right_bracket_tree = ParseTree::new(
        factory.clone(),
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::RightBracket)),
        vec![end_tree(factory.clone())],
        noop_fn,
    );

    let right_bracket_node = graph.add_tree(&right_bracket_tree);

    let number_node = graph.add_node(StatementKind::Number(NumberKind::Float), add_number);

    let comma_node = graph.add_node(
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::Comma)),
        noop_fn,
    );

    graph.add_child(comma_node, number_node);
    graph.add_child(number_node, comma_node);
    graph.add_child(number_node, right_bracket_node);

    let repetition_tree = graph.build_from(factory.clone(), number_node);

    let array_tree = {
        let mut tree = ParseTree::new(
            factory.clone(),
            StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::LeftBracket)),
            vec![right_bracket_tree],
            noop_fn,
        );

        tree.push_child(repetition_tree);
        tree
    };

    let tree = expect_words!(
        factory.clone();
        "The",
        "sum",
        "of",
        "all",
        "the",
        "elements",
        "of",
        "the",
        "array";
        vec![
            ParseTree::new(
                factory.clone(),
                StatementKind::Symbol(Symbol::Punctuation(Punctuation::Colon)),
                vec![array_tree],
                noop_fn,
            ),
        ];
        noop_fn
    );

    let inputs = [
        "The sum of all the elements of the array: [1, 2, 3, 4] is 10.",
        "The sum of all the elements of the array: [1, 2, 3, 4] is 10. // total",
        "The sum of all the elements of the array: [5, 10, 15] is 30.",
        "The sum of all the elements of the array: [5, 10, 15] is 30. // verified",
        "The sum of all the elements of the array: [] is 0.",
        "The sum of all the elements of the array: [] is 0. // verified",
        "The sum of all the elements of the array: [] is 0. //
        The sum of all the elements of the array: [5.0, 10, 15] is 30. // Multi-line",
    ];

    let lexer = Lexer::<1>::new("");
    let mut parser = Parser::<TestSumContext, 1, 1>::new(lexer, tree);

    for input in inputs {
        parser.update_input(input);

        let res = parser.parse();

        assert!(
            res.is_ok(),
            "expected success for {:?}, got: {:?}",
            input,
            res.unwrap_err()
        );
    }
}
