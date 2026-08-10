// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::{parse, parser_errors::FindElementError};
use std::fs;
use std::path::Path;

mod test_parser;

#[rstest]
#[case("lists", "ok_field_list.rst", "ok_field_list.yaml")]
#[case("lists", "ok_bullet_list.rst", "ok_bullet_list.yaml")]
#[case("lists", "ok_compact_bullet_list.rst", "ok_compact_bullet_list.yaml")]
#[case(
    "lists",
    "nok_missing_blanklines_bulletlist1.rst",
    "nok_missing_blanklines_bulletlist1.yaml"
)]
fn parses_directives_and_matches_yaml_fixture(
    #[case] directory: &str,
    #[case] rst_filename: &str,
    #[case] yaml_filename: &str,
) {
    rst_vs_yaml!(directory, rst_filename, yaml_filename)
}

#[test]
fn rejects_mixed_bullet_list_markers_fixture() {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists")
        .join("nok_mixed_bullet_list_markers.rst");
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, FindElementError::ListStyleError { .. }));
}
