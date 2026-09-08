// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::lexer::tokenize;
use rstu_parser::token::{Token, TokenKind as TK};
use std::fs;
use std::path::Path;

#[test]
fn tokenize_ok_mixed_lorem_ipsum_file() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/mixed/ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read mixed lorem ipsum test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TK, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TK::BlankLine, "\n"),
        (TK::Word, "Lorem"),
        (TK::Spaces, " "),
        (TK::Word, "Ipsum"),
        (TK::Spaces, " "),
        (TK::Word, "Heading"),
        (TK::NewLine, "\n"),
        (TK::Separator, "==================="),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::DoubleDot, ".."),
        (TK::Spaces, " "),
        (TK::Word, "note"),
        (TK::DoubleColon, "::"),
        (TK::NewLine, "\n"),
        (TK::Indent, "   "),
        (TK::Word, "Lorem"),
        (TK::Spaces, " "),
        (TK::Word, "ipsum"),
        (TK::Spaces, " "),
        (TK::Word, "dolor"),
        (TK::Spaces, " "),
        (TK::Word, "sit"),
        (TK::Spaces, " "),
        (TK::Word, "amet"),
        (TK::Punctuation, ","),
        (TK::Spaces, " "),
        (TK::Word, "consectetur"),
        (TK::Spaces, " "),
        (TK::Word, "adipiscing"),
        (TK::Spaces, " "),
        (TK::Word, "elit"),
        (TK::Punctuation, "."),
        (TK::NewLine, "\n"),
        (TK::Word, "Vivamus"),
        (TK::Spaces, " "),
        (TK::Word, "lacinia"),
        (TK::Spaces, " "),
        (TK::Word, "odio"),
        (TK::Spaces, " "),
        (TK::Word, "vitae"),
        (TK::Spaces, " "),
        (TK::Word, "vestibulum"),
        (TK::Spaces, " "),
        (TK::Word, "vestibulum"),
        (TK::Punctuation, "."),
        (TK::NewLine, "\n"),
        (TK::Word, "Cras"),
        (TK::Spaces, " "),
        (TK::Word, "venenatis"),
        (TK::Spaces, " "),
        (TK::Word, "euismod"),
        (TK::Spaces, " "),
        (TK::Word, "malesuada"),
        (TK::Punctuation, "."),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::Dedent, "   "),
        (TK::DoubleDot, ".."),
        (TK::Spaces, " "),
        (TK::Word, "comment"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::StrongStart, "**"),
        (TK::Word, "end"),
        (TK::Spaces, " "),
        (TK::Word, "of"),
        (TK::Spaces, " "),
        (TK::Word, "file"),
        (TK::StrongEnd, "**"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}

#[test]
fn tokenize_ok_indentation() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/indentation/ok_indent_blankline.rst");
    let contents = fs::read_to_string(path).expect("failed to read test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TK, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TK::BlankLine, "\n"),
        (TK::Word, "First"),
        (TK::Spaces, " "),
        (TK::Word, "line"),
        (TK::NewLine, "\n"),
        (TK::Indent, "  "),
        (TK::Word, "Indented1"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::Word, "Still"),
        (TK::Spaces, " "),
        (TK::Word, "Indented"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::Dedent, "  "),
        (TK::Word, "Dedented1"),
        (TK::Punctuation, "."),
        (TK::NewLine, "\n"),
        (TK::Indent, "  "),
        (TK::Word, "Indented2"),
        (TK::NewLine, "\n"),
        (TK::Dedent, "  "),
        (TK::Word, "Dedented2"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}

#[test]
fn tokenize_ok_indentation_2blanklines() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/indentation/ok_indent_2blankline.rst");
    let contents = fs::read_to_string(path).expect("failed to read test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TK, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TK::BlankLine, "\n"),
        (TK::Word, "First_line"),
        (TK::NewLine, "\n"),
        (TK::Indent, "  "),
        (TK::Word, "Indented"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::BlankLine, "\n"),
        (TK::Word, "Still"),
        (TK::Spaces, " "),
        (TK::Word, "Indented"),
        (TK::NewLine, "\n"),
        (TK::Dedent, "  "),
        (TK::Word, "Dedented"),
        (TK::NewLine, "\n"),
        (TK::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}
