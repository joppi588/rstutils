// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT


// Lexer development on hold

use rstu_parser::lexer::tokenize;
use std::fs;
use std::path::Path;

#[test]
fn tokenize_lorem_ipsum_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read lorem ipsum test file");

    let tokens = tokenize(&contents);

    assert_eq!(tokens.len(), 19);
    assert_eq!(tokens.first().map(String::as_str), Some("Lorem Ipsum Heading"));
    assert_eq!(tokens.last().map(String::as_str), Some(""));
}
