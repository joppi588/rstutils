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
    let kinds: Vec<TokenKind> = tokens.iter().map(|token| token.kind).collect();
    let expected_kinds = vec![
        TokenKind::Word,
        TokenKind::LiteralString, // Spaces
        TokenKind::Word,
        TokenKind::NewLine,
        TokenKind::HeadingUnderline,
        TokenKind::LiteralString, // Blankline
        TokenKind::LiteralString,
        TokenKind::DoubleDot,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::NewLine,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::LiteralString,
        TokenKind::DoubleDot,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::Word,
        TokenKind::LiteralString,
        TokenKind::LiteralString,
    ];

    assert_eq!(kinds, expected_kinds);
}
