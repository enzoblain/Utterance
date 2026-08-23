# Utterance

[![CI](https://github.com/enzoblain/Utterance/actions/workflows/ci.yml/badge.svg)](https://github.com/enzoblain/Utterance/actions/workflows/ci.yml)
[![docs.rs](https://docs.rs/utterance/badge.svg)](https://docs.rs/utterance)
[![Crates.io](https://img.shields.io/crates/v/utterance.svg)](https://crates.io/crates/utterance)
[![MSRV](https://img.shields.io/badge/MSRV-1.96-blue.svg)](#minimum-supported-rust-version)

> [!NOTE]
> Utterance is the linguistic term for a complete unit of speech, a sentence, a command, or any meaningful spoken expression. The name reflects the goal of this project: making it easy to build domain-specific languages that read like natural English instead of traditional programming syntax.

## Overview

`Utterance` is a parsing framework designed for creating human-readable domain-specific languages (DSLs). Instead of focusing on symbolic grammars alone, `utterance` allows developers to describe languages that resemble natural English while retaining the performance and safety of Rust.

The library combines a lexer, parser, semantic callbacks, and syntax highlighting into a single framework. It is suitable for command languages, configuration formats, educational languages, scripting languages, and any project where readability is a priority.

## Installation

Add `utterance` to your `Cargo.toml`:

```toml
[dependencies]
utterance = "0.2.6"
```

Or using Cargo:

```bash
cargo add utterance
```

## Features

- Build grammars by composing parse trees.
- Express recursive grammars with cyclic graph builders.
- Execute callbacks while parsing.
- Store and validate application state through a custom context.
- Built-in lexer.
- Lightweight and dependency-friendly.
- Detailed parse errors.
- Syntax highlighting support.

## Quick examples

### Basic grammar

Define grammars by composing parse trees. Each node matches a token and points to the possible continuations.

```rust
use utterance::lexer::Lexer;
use utterance::parser::expectation::{StatementKind, WordKind};
use utterance::parser::tree::{ParseContext, ParseTree, ParseTreeFactory, noop_fn};
use utterance::parser::{CustomParseError, Parser};
use utterance::syntax::HighlightKind;
use utterance::{empty_children, expect_words, tree_fn};

#[derive(Clone, Default, Debug)]
struct Context {
    pub name: Option<String>,
}

impl ParseContext for Context {
    type Error = CustomParseError;
}

fn main() {
    // Create a factory used to avoid re-creating common trees
    let factory = ParseTreeFactory::<Context>::new();
    let tree = expect_words!(
        factory.clone();
        "Hello",
        "my",
        "name",
        "is";
        vec![
            ParseTree::new(
                factory.clone(),
                StatementKind::Word(WordKind::Custom{ kind: HighlightKind::Special }),
                empty_children!(),
                tree_fn!(Word(context, name) => {
                    context.name = Some(name.to_string());

                    Ok(())
                }),
            ),
        ];
        noop_fn
    )
    .into_line(); // Accept a complete sentence starting with an uppercase letter and terminated by a period, followed by either a newline (or end) or a comment.

    let lexer = Lexer::<1>::new("Hello my name is Alice.");
    let mut parser = Parser::<Context, 1, 1>::new(lexer, tree);

    assert!(parser.parse().is_ok());
    assert_eq!(parser.context().name.as_deref(), Some("Alice"));
}
```

Valid inputs:

```text
Hello my name is utterance.
Hello my name is Alice.
```

### Cyclic trees

Arrays are a common example of cyclic grammars. The parser can describe an arbitrary number of elements without manually duplicating rules.

```rust
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

fn main() {
    let factory = ParseTreeFactory::<NoopContext>::new();

    // Represent the grammar as a graph, allowing cyclic productions.
    let mut graph = GraphBuilder::<NoopContext>::new();

    let right_bracket_tree = ParseTree::new(
        factory.clone(),
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::RightBracket)),
        [factory.line_terminator()],
        noop_fn,
    );

    let left_bracket = graph.add_node(
        StatementKind::Symbol(Symbol::SpecialChar(SpecialChar::LeftBracket)),
        noop_fn,
    );
    let right_bracket = graph.add_tree(&right_bracket_tree);
    let number = graph.add_node(StatementKind::Number(NumberKind::Float), noop_fn);
    let comma = graph.add_node(
        StatementKind::Symbol(Symbol::Punctuation(Punctuation::Comma)),
        noop_fn,
    );

    // Define every valid transition after reading an array element
    graph.add_child(left_bracket, number);
    graph.add_child(left_bracket, right_bracket);
    graph.add_child(number, right_bracket);
    graph.add_child(number, comma);
    graph.add_child(comma, number);

    // Transform the grammar graph into a parse tree
    let tree = graph.build_from(factory.clone(), left_bracket);
    let tree = ParseTree::noop(factory, [tree]); // Creating a ParseTree instead of Rc<ParseTree>

    let lexer = Lexer::<1>::new("[15, 1.0]");
    let mut parser = Parser::<NoopContext, 1, 1>::new(lexer, tree);

    assert!(parser.parse().is_ok())
}
```

Valid inputs:

```text
[]
[1]
[1, 2.1]
[1, -5.2, 365]
[1, 2.4434232, -332, 4.0]
...
```

## Why Utterance?

Unlike traditional parser combinators or parser generators, Utterance is designed for building DSLs that resemble natural language. Grammars are described as parse trees, semantic actions are integrated directly into the parsing process, and recursive constructs can be expressed naturally using cyclic grammar graphs.

## License

Licensed under the MIT License.
