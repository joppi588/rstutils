// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parse;
use rstu_parser::FindElementError;
use std::fs;
use std::path::Path;

fn section_data_path(filename: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/sections")
        .join(filename)
}

#[rstest]
#[case("ok_three_sections.rst", "ok_three_sections.yaml")]
#[case("ok_sections_style.rst", "ok_sections_style.yaml")]
fn parses_sections_and_matches_yaml_fixture(
    #[case] rst_filename: &str,
    #[case] yaml_filename: &str,
) {
    let rst_path = section_data_path(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read sections test file: {rst_filename}"));

    let parsed = parse(&rst_contents).expect("expected parse to succeed");
    let actual_yaml = parsed
        .to_yaml()
        .expect("failed to serialize parse output to yaml");

    let expected_path = section_data_path(yaml_filename);
    let expected_yaml = fs::read_to_string(&expected_path)
        .unwrap_or_else(|_| panic!("failed to read expected yaml fixture: {yaml_filename}"));

    let actual_value: serde_yaml::Value =
        serde_yaml::from_str(&actual_yaml).expect("failed to parse generated yaml");
    let expected_value: serde_yaml::Value =
        serde_yaml::from_str(&expected_yaml).expect("failed to parse expected yaml fixture");

    assert_eq!(
        actual_value, expected_value,
        "unexpected parse output for {rst_filename}"
    );
}

#[test]
fn test_missing_closing() {
    let path = section_data_path("nok_sections_missing_closing.rst");
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
    let path = section_data_path("nok_sections_unbalanced_style.rst");
    let contents =
        fs::read_to_string(path).unwrap_or_else(|_| panic!("failed to read sections test file"));

    let result = parse(&contents);

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
