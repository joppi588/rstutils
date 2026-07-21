// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::lexer::tokenize;
use rstu_parser::try_find_section_header;
use std::fs;
use std::path::Path;

#[rstest]
#[case("ok_three_sections.rst")]
#[case("ok_sections_style.rst")]
fn finds_all_section_headers(#[case] filename: &str) {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sections")
        .join(filename);
    let contents = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("failed to read sections test file: {filename}"));

    let tokens = tokenize(&contents);
    let mut start_at = 0;
    let mut spans = Vec::new();

    while let Some((start, end)) =
        try_find_section_header(&tokens, start_at).expect("failed to scan section header")
    {
        spans.push((start, end));
        start_at = end + 1;
    }

    assert_eq!(
        spans.len(),
        3,
        "expected three section headers in {filename}"
    );

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

    assert_eq!(
        titles,
        vec!["Heading 1\n", "Heading 2\n", "Heading 3\n"],
        "unexpected extracted titles in {filename}"
    );
}
