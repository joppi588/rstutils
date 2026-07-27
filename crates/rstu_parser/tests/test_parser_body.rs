// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::parse;
use std::fs;
use std::path::Path;

fn body_data_path(filename: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/body")
        .join(filename)
}

#[rstest]
#[case("ok_strong.rst", "ok_strong.yaml")]
/// GIVEN a body fixture with inline strong markup
/// WHEN parsing and serializing the AST to YAML
/// THEN generated and expected YAML trees are equivalent
fn parses_body_and_matches_yaml_fixture(#[case] rst_filename: &str, #[case] yaml_filename: &str) {
    let rst_path = body_data_path(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read body test file: {rst_filename}"));

    let parsed = parse(&rst_contents).expect("expected parse to succeed");
    let actual_yaml = AstNode::to_yaml(&parsed).expect("failed to serialize parse output to yaml");

    let expected_path = body_data_path(yaml_filename);
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
