// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::lexer::tokenize;
use rstu_parser::token::{Token, TokenKind};
use rstu_parser::try_match_section_header;
use rstu_parser::FindElementError;
use std::fs;
use std::path::Path;

fn extract_title_from_tokens(tokens: &[Token], marker_index: usize) -> String {
    if marker_index == 0 {
        return String::new();
    }

    // For suffix-style headers, the marker line starts after the title line's newline.
    // Step back into the previous line and then collect that line's tokens.
    let mut line_end = marker_index;
    while line_end > 0
        && matches!(
            tokens[line_end - 1].kind,
            TokenKind::NewLine | TokenKind::BlankLine
        )
    {
        line_end -= 1;
    }

    if line_end == 0 {
        return String::new();
    }

    let mut line_start = line_end;
    while line_start > 0
        && !matches!(
            tokens[line_start - 1].kind,
            TokenKind::NewLine | TokenKind::BlankLine
        )
    {
        line_start -= 1;
    }

    let mut title = String::new();
    for token in &tokens[line_start..line_end] {
        title.push_str(&token.lexeme);
    }
    title.push('\n');
    title
}

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
    let mut sections = Vec::new();

    while start_at < tokens.len() {
        let scan_tokens = tokens[start_at..].to_vec();
        match try_match_section_header(&scan_tokens, 0).expect("failed to scan section header") {
            Some((section, next_start)) => {
                sections.push((start_at, section));
                start_at += next_start.max(1);
            }
            None => {
                start_at += 1;
            }
        }
    }

    assert_eq!(
        sections.len(),
        3,
        "expected three section headers in {filename}"
    );

    let titles: Vec<String> = sections
        .iter()
        .map(|(marker_index, section)| {
            let node_title = section
                .children
                .first()
                .and_then(|title_node| title_node.text.clone())
                .map(|title| title.trim_start_matches('\n').to_string())
                .unwrap_or_default();

            if node_title.is_empty() {
                extract_title_from_tokens(&tokens, *marker_index)
            } else {
                node_title
            }
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
    let mut result = Ok(None);
    for (index, token) in tokens.iter().enumerate() {
        if !matches!(
            token.kind,
            TokenKind::SectionTitlePrefix | TokenKind::SectionTitleSuffix
        ) {
            continue;
        }

        result = try_match_section_header(&tokens, index);
        if result.is_err() {
            break;
        }
    }

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
    let (first_section, next_start) = try_match_section_header(&tokens, 0)
        .expect("failed to scan section header")
        .unwrap();
    assert_eq!(
        first_section
            .children
            .first()
            .and_then(|title_node| title_node.text.as_deref()),
        Some("\nHeading 1\n"),
        "expected first section to be parsed before mismatch"
    );

    let mut start_at = next_start;
    let mut result = Ok(None);
    while start_at < tokens.len() {
        let scan_tokens = tokens[start_at..].to_vec();
        result = try_match_section_header(&scan_tokens, 0);
        match &result {
            Err(_) => break,
            Ok(Some((_, advance_by))) => start_at += (*advance_by).max(1),
            Ok(None) => start_at += 1,
        }
    }

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
