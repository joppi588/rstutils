// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::lexer::tokenize;
use rstu_parser::try_match_section_header;
use rstu_parser::FindElementError;
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
        try_match_section_header(&tokens, start_at).expect("failed to scan section header")
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

#[test]
fn test_missing_closing() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sections/nok_sections_missing_closing.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let tokens = tokenize(&contents);
    let result = try_match_section_header(&tokens, 10);

    assert!(
        result.is_err(),
        "expected FindElementError::SectionTitleMissingClosingAfterOpening error"
    );

    if let Err(err) = result {
        let err_string = format!("{:?}", err);
        assert!(
            err_string.contains("SectionTitleMissingClosingAfterOpening"),
            "expected SectionTitleMissingClosingAfterOpening error, got: {err_string}"
        );
    }
}

#[test]
fn test_unbalanced_section_style() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sections/nok_sections_unbalanced_style.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let tokens = tokenize(&contents);
    let (first_start, first_end) = try_match_section_header(&tokens, 0)
        .expect("failed to scan section header")
        .unwrap();
    assert_eq!(
        &tokens[first_start].lexeme, "Heading",
        "expected first section to be parsed before mismatch"
    );

    let result = try_match_section_header(&tokens, first_end + 1);

    match result {
        Err(FindElementError::SectionTitleUnbalancedStyle {
            opening_style,
            closing_style,
            ..
        }) => {
            assert_eq!(opening_style, "---------");
            assert_eq!(closing_style, "=========");
        }
        other => panic!("expected SectionTitleUnbalancedStyle error, got: {other:?}"),
    }
}
