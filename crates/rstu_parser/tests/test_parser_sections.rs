// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::parse;
use rstu_parser::parser_errors::FindElementError;
use std::fs;
use std::path::Path;
mod test_parser;

fn data_path(directory: &str, filename: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(directory)
        .join(filename)
}

#[rstest]
#[case("sections", "ok_sections_type1.rst", "ok_sections_type1.yaml")]
#[case("sections", "ok_sections_type2.rst", "ok_sections_type2.yaml")]
fn parses_sections_and_matches_yaml_fixture(
    #[case] directory: &str,
    #[case] rst_filename: &str,
    #[case] yaml_filename: &str,
) {
    rst_vs_yaml!(directory, rst_filename, yaml_filename)
}

#[test]
fn test_missing_closing() {
    let path = data_path("sections", "nok_sections_missing_closing.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let result = parse(&contents);

    match result {
        Err(FindElementError::SectionTitleMissingClosingAfterOpening { .. }) => {}
        other => panic!("expected SectionTitleMissingClosingAfterOpening error, got: {other:?}"),
    }
}

#[test]
fn test_unbalanced_section_style() {
    let path = data_path("sections", "nok_sections_unbalanced_style.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let result = parse(&contents);

    match result {
        Err(FindElementError::SectionTitleUnbalancedStyle {
            opening_style,
            closing_style,
            ..
        }) => {
            assert_eq!(opening_style, "-");
            assert_eq!(closing_style, "=");
        }
        other => panic!("expected SectionTitleUnbalancedStyle error, got: {other:?}"),
    }
}
