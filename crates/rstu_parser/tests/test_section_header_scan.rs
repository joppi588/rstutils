// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::lexer::tokenize;
use rstu_parser::token::TokenKind;
use rstu_parser::try_find_section_header;
use std::fs;
use std::path::Path;

#[test]
fn finds_all_section_headers_in_ok_three_sections() {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/sections/ok_three_sections.rst");
    let contents = fs::read_to_string(path).expect("failed to read three sections test file");

    let tokens = tokenize(&contents);
    let mut start_at = 0;
    let mut spans = Vec::new();

    while let Some((start, end)) =
        try_find_section_header(&tokens, start_at).expect("failed to scan section header")
    {
        spans.push((start, end));
        start_at = end + 1;
    }

    assert_eq!(spans.len(), 3, "expected three section headers");

    let titles: Vec<String> = spans
        .iter()
        .map(|(start, end)| {
            let mut title = String::new();
            for token in &tokens[*start..*end] {
                title.push_str(&token.lexeme);
            }
            title
        })
        .collect();

    assert_eq!(titles, vec!["Heading 1\n", "Heading 2\n", "Heading 3\n"]);
}
