// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::{parse, parser_errors::ParserError};
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;
#[rstest]
// TODO: Activate tests
#[case("docutils_bullet_00")]
#[case("docutils_bullet_01")]
#[case("docutils_bullet_02")]
#[case("docutils_bullet_03")]
#[case("docutils_bullet_04")]
#[case("docutils_bullet_07")]
#[case("ok_bullet_list")]
#[case("ok_compact_bullet_list")]
#[case("ok_nested_bullet_list")]
// #[case("docutils_bullet_09")]
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
#[case("nok_empty_item_no_blank")]
#[case("ok_enumerated_list")]
#[case("ok_no_blank_lines_between_items")]
#[case("ok_different_enumeration_sequences")]
#[case("ok_nested_enumerated_lists")]
#[case("ok_auto_enumerator")]
#[case("ok_potentially_ambiguous")]
#[case("ok_definitely_ambiguous")]
#[case("ok_different_enumeration_formats")]
#[case("ok_loweralpha_auto")]
#[case("ok_lowerroman_auto")]
#[case("ok_non_marker_period")]
fn parse_enumerated_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/enumerated_list", test_case)
}

#[rstest]
#[case("nok_unindented_enumerated_continuation.rst")]
fn rejects_docutils_enumerated_list_errors(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/enumerated_list")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}

// TODO
// NOT IMPLEMENTED: Docutils emits system messages for ordinal validation.
// #[case("docutils_scrambled_sequences")]
// #[case("docutils_skipping_item")]
// #[case("docutils_nonordinal_starts")]
// NOT IMPLEMENTED: Roman numeral validation and recovery differ.
// #[case("docutils_bad_roman_numerals")]
// NOT IMPLEMENTED: Docutils recovers misaligned continuation lines as warnings/block quotes.
// #[case("docutils_misaligned_multiline_items")]
// NOT IMPLEMENTED: Docutils leaves the later markers as paragraph text after a warning.
// #[case("docutils_mixed_auto_and_explicit")]
// NOT IMPLEMENTED: Non-breaking-space handling is not supported by the current lexer.
// #[case("docutils_nonbreaking_space_workaround")]
// NOT IMPLEMENTED: The remaining indentation matrix includes unsupported recovery cases.
// #[case("ok_enumerated_item_indentation")]
// NOT IMPLEMENTED: A list cannot currently start with an auto-enumerator.
// #[case("ok_auto_only")]
// #[case("ok_multiline_enumerated_items")]

#[rstest]
#[case("bodies_next_line")]
#[case("multiline_aligned")]
#[case("multiline_not_lined_up")]
#[case("multiple_arguments")]
#[case("multiple_body_elements")]
#[case("nested_one_line")]
#[case("ok_field_list")]
#[case("oneliners_no_blank")]
// NOT IMPLEMENTED:
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

#[rstest]
#[case("nok_empty_item_no_blank.rst")]
#[case("nok_list_end_no_blank.rst")]
fn rejects_docutils_field_list_end(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/field_list")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}
