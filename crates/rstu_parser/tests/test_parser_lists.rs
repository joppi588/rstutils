// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::{parse, parser_errors::ParserError};
use std::fs;
use std::path::Path;

mod test_parser;
#[rstest]
// TODO: Activate tests
#[rstest]
#[case("ok_bullet_list")]
#[case("ok_compact_bullet_list")]
#[case("docutils_bullet_00")]
#[case("docutils_bullet_01")]
// #[case("docutils_bullet_02")] compound body
// #[case("docutils_bullet_03")] compound body
// #[case("docutils_bullet_04")]
// #[case("docutils_bullet_09")]
#[case("docutils_bullet_07")]
fn parse_bullet_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/bullet_list", test_case)
}

#[rstest]
#[case("docutils_bullet_05.rst")]
fn rejects_docutils_bullet_list_style(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/bullet_list")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListStyleError { .. }));
}

#[rstest]
#[case("docutils_bullet_06.rst")]
#[case("docutils_bullet_08.rst")]
fn rejects_docutils_bullet_list_end(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/bullet_list")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}

#[rstest]
#[case("ok_field_list")]
#[case("bodies_next_line")]
#[case("multiline_aligned")]
#[case("multiline_not_lined_up")]
#[case("multiple_arguments")]
#[case("oneliners_no_blank")]
// NOT IMPLEMENTED:
// #[case("multiple_body_elements")]
// #[case("nested_one_line")]
// #[case("inline_markup_in_name")]
// #[case("bad_inline_markup")]
// #[case("edge_cases")]
// #[case("embedded_colons_comment_split")]
// #[case("embedded_colons_interpreted_text")]

fn parse_field_list(#[case] test_case: &str) {
    // GIVEN field-list examples
    // WHEN we parse and compare them against YAML snapshots
    // THEN this acts as a compatibility porting test surface (expected to fail for now)

    rst_vs_yaml!("lists/field_list", test_case);
}

#[test]
fn field_lists_doesnt_end_in_blankline() {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/field_list")
        .join("nok_empty_item_no_blank.rst");
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}
