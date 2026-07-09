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

    assert!(!tokens.is_empty());
    assert_eq!(tokens.first().map(|t| t.kind), Some(TokenKind::LiteralString));
    assert_eq!(tokens.last().map(|t| t.kind), Some(TokenKind::LiteralString));
}
