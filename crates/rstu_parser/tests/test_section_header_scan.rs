// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::lexer::tokenize;
use rstu_parser::token::{Token, TokenKind};
use rstu_parser::FindElementError;
use rstu_parser::{try_match_section_header_prefix, try_match_section_header_suffix};
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
        match tokens[start_at].kind {
            TokenKind::SectionTitlePrefix => {
                let (section, next_start) = try_match_section_header_prefix(&tokens, start_at)
                    .expect("failed to scan section header prefix");
                sections.push((start_at, section));
                start_at = next_start;
            }
            TokenKind::SectionTitleSuffix => {
                let (section, next_start) = try_match_section_header_suffix(&tokens, start_at)
                    .expect("failed to scan section header suffix");
                sections.push((start_at, section));
                start_at = next_start;
            }
            _ => {
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
    let mut captured_error: Option<FindElementError> = None;
    for (index, token) in tokens.iter().enumerate() {
        let result = match token.kind {
            TokenKind::SectionTitlePrefix => try_match_section_header_prefix(&tokens, index),
            TokenKind::SectionTitleSuffix => try_match_section_header_suffix(&tokens, index),
            _ => continue,
        };

        if let Err(err) = result {
            captured_error = Some(err);
            break;
        }
    }

    match captured_error {
        Some(FindElementError::SectionTitleMissingClosingAfterOpening { .. }) => {}
        other => panic!("expected SectionTitleMissingClosingAfterOpening error, got: {other:?}"),
    }
}

#[test]
fn test_unbalanced_section_style() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sections/nok_sections_unbalanced_style.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let tokens = tokenize(&contents);
    let (first_section, mut start_at) = try_match_section_header_prefix(&tokens, 0)
        .expect("failed to scan first section header prefix");
    assert_eq!(
        first_section
            .children
            .first()
            .and_then(|title_node| title_node.text.as_deref()),
        Some("\nHeading 1\n"),
        "expected first section to be parsed before mismatch"
    );

    let mut captured_error: Option<FindElementError> = None;
    while start_at < tokens.len() {
        let result = match tokens[start_at].kind {
            TokenKind::SectionTitlePrefix => try_match_section_header_prefix(&tokens, start_at),
            TokenKind::SectionTitleSuffix => try_match_section_header_suffix(&tokens, start_at),
            _ => {
                start_at += 1;
                continue;
            }
        };

        match result {
            Ok((_, next_start)) => start_at = next_start,
            Err(err) => {
                captured_error = Some(err);
                break;
            }
        }
    }

    match captured_error {
        Some(FindElementError::SectionTitleUnbalancedStyle {
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
