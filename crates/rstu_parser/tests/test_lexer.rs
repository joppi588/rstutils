// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::lexer::tokenize;
use rstu_parser::token::{Token, TokenKind};
use std::fs;
use std::path::Path;

#[test]
fn tokenize_ok_mixed_lorem_ipsum_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read mixed lorem ipsum test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TokenKind, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TokenKind::BlankLine, "\n"),
        (TokenKind::Word, "Lorem"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "Ipsum"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "Heading"),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Separator, "==================="),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
        (TokenKind::DoubleDot, ".."),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "note"),
        (TokenKind::DoubleColon, "::"),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Indent, "   "),
        (TokenKind::Word, "Lorem"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "ipsum"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "dolor"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "sit"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "amet"),
        (TokenKind::Punctuation, ","),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "consectetur"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "adipiscing"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "elit"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Indent, "   "),
        (TokenKind::Word, "Vivamus"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "lacinia"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "odio"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "vitae"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "vestibulum"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "vestibulum"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Indent, "   "),
        (TokenKind::Word, "Cras"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "venenatis"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "euismod"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "malesuada"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
        (TokenKind::DoubleDot, ".."),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "comment"),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
        (TokenKind::Strong, "**"),
        (TokenKind::Word, "end"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "of"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "file"),
        (TokenKind::Strong, "**"),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}

#[test]
fn tokenize_ok_note_simple_file() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/directives/ok_note_simple.rst");
    let contents =
        fs::read_to_string(path).expect("failed to read simple note directive test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TokenKind, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TokenKind::BlankLine, "\n"),
        (TokenKind::DoubleDot, ".."),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "note"),
        (TokenKind::DoubleColon, "::"),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Indent, "   "),
        (TokenKind::Word, "This"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "is"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "a"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "simple"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "note"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "directive"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}

#[test]
fn tokenize_ok_comment_simple_file() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/directives/ok_comment_simple.rst");
    let contents =
        fs::read_to_string(path).expect("failed to read simple comment directive test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TokenKind, &str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
        (TokenKind::BlankLine, "\n"),
        (TokenKind::DoubleDot, ".."),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "This"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "is"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "a"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "simple"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "comment"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::Indent, "   "),
        (TokenKind::Word, "It"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "should"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "be"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "ignored"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "by"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "the"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "parser"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "as"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "visible"),
        (TokenKind::Spaces, " "),
        (TokenKind::Word, "content"),
        (TokenKind::Punctuation, "."),
        (TokenKind::NewLine, "\n"),
        (TokenKind::BlankLine, "\n"),
    ];

    assert_eq!(kinds, expected_kinds);
}
