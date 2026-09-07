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
#[case("ok_bullet_list.rst", "ok_bullet_list.yaml")]
#[case("ok_compact_bullet_list.rst", "ok_compact_bullet_list.yaml")]
#[case(
    "nok_missing_blanklines_bulletlist1.rst",
    "nok_missing_blanklines_bulletlist1.yaml"
)]
fn parse_bullet_list(#[case] rst_filename: &str, #[case] yaml_filename: &str) {
    rst_vs_yaml!("lists/bullet_list", rst_filename, yaml_filename)
}

#[rstest]
#[case("docutils_bullet_00.rst", "docutils_bullet_00.yaml")]
#[case("docutils_bullet_01.rst", "docutils_bullet_01.yaml")]
// #[case("docutils_bullet_02.rst", "docutils_bullet_02.yaml")] compound body
// #[case("docutils_bullet_03.rst", "docutils_bullet_03.yaml")] compound body
// #[case("docutils_bullet_04.rst", "docutils_bullet_04.yaml")]
#[case("docutils_bullet_07.rst", "docutils_bullet_07.yaml")]
// #[case("docutils_bullet_09.rst", "docutils_bullet_09.yaml")]
fn parse_docutils_bullet_list(#[case] rst_filename: &str, #[case] yaml_filename: &str) {
    rst_vs_yaml!("lists/bullet_list", rst_filename, yaml_filename)
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
#[case("ok_field_list.rst", "ok_field_list.yaml")]
#[case("bodies_next_line.rst", "bodies_next_line.yaml")]
#[case("multiline_aligned.rst", "multiline_aligned.yaml")]
#[case("multiline_not_lined_up.rst", "multiline_not_lined_up.yaml")]
#[case("multiple_arguments.rst", "multiple_arguments.yaml")]
#[case("oneliners_no_blank.rst", "oneliners_no_blank.yaml")]
// NOT IMPLEMENTED:
// #[case("multiple_body_elements.rst", "multiple_body_elements.yaml")]
// #[case("nested_one_line.rst", "nested_one_line.yaml")]
// #[case("inline_markup_in_name.rst", "inline_markup_in_name.yaml")]
// #[case("bad_inline_markup.rst", "bad_inline_markup.yaml")]
// #[case("edge_cases.rst", "edge_cases.yaml")]
// #[case(
//     "embedded_colons_comment_split.rst",
//     "embedded_colons_comment_split.yaml"
// )]
// #[case(
//     "embedded_colons_interpreted_text.rst",
//     "embedded_colons_interpreted_text.yaml"
// )]

fn parse_field_list(#[case] rst_filename: &str, #[case] yaml_filename: &str) {
    // GIVEN field-list examples
    // WHEN we parse and compare them against YAML snapshots
    // THEN this acts as a compatibility porting test surface (expected to fail for now)

    rst_vs_yaml!("lists/field_list", rst_filename, yaml_filename);
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
