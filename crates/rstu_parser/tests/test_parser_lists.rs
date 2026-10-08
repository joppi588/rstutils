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
#[case("bullet_00")]
#[case("bullet_01")]
#[case("bullet_02")]
#[case("bullet_03")]
#[case("bullet_04")]
#[case("bullet_07")]
#[case("bullet_list")]
#[case("compact_bullet_list")]
#[case("nested_bullet_list")]
// TODO: Activate tests
// #[case("bullet_09")] Unicode bullets
fn parse_bullet_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/bullet_list/ok", test_case)
}

#[rstest]
#[case("bullet_05.rst")]
fn rejects_docutils_bullet_list_style(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/bullet_list/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListStyleError { .. }));
}

#[rstest]
#[case("bullet_06.rst")]
#[case("bullet_08.rst")]
fn rejects_docutils_bullet_list_end(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/bullet_list/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}

#[rstest]
#[case("auto_enumerator")]
#[case("auto_only")]
#[case("definitely_ambiguous")]
#[case("different_enumeration_formats")]
#[case("different_enumeration_sequences")]
#[case("empty_item_no_blank")]
#[case("enumerated_list")]
#[case("loweralpha_auto")]
#[case("lowerroman_auto")]
#[case("mixed_auto_and_explicit")]
#[case("nested_enumerated_lists")]
#[case("no_blank_lines_between_items")]
#[case("non_marker_period")]
#[case("potentially_ambiguous")]
fn parse_enumerated_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/enumerated_list/ok", test_case)
}

#[rstest]
#[case("unindented_enumerated_continuation.rst")]
fn rejects_docutils_enumerated_list_errors(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/enumerated_list/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}

// TODO
// NOT IMPLEMENTED: Docutils emits system messages for ordinal validation.
// #[case("scrambled_sequences")]
// #[case("skipping_item")]
// #[case("nonordinal_starts")]
// NOT IMPLEMENTED: Roman numeral validation and recovery differ.
// #[case("bad_roman_numerals")]
// NOT IMPLEMENTED: Docutils recovers misaligned continuation lines as warnings/block quotes.
// #[case("misaligned_multiline_items")]
// NOT IMPLEMENTED: Non-breaking-space handling is not supported by the current lexer.
// #[case("nonbreaking_space_workaround")]
// #[case("enumerated_item_indentation")]
// #[case("multiline_enumerated_items")]

#[rstest]
#[case("bodies_next_line")]
#[case("multiline_aligned")]
#[case("multiline_not_lined_up")]
#[case("multiple_arguments")]
#[case("multiple_body_elements")]
#[case("nested_one_line")]
#[case("field_list")]
#[case("oneliners_no_blank")]
// TODO: NOT IMPLEMENTED:
// #[case("inline_markup_in_name")]
// #[case("bad_inline_markup")]
// #[case("edge_cases")]
// #[case("embedded_colons_comment_split")]
// #[case("embedded_colons_interpreted_text")]

fn parse_field_list(#[case] test_case: &str) {
    // GIVEN field-list examples
    // WHEN we parse and compare them against YAML snapshots
    // THEN this acts as a compatibility porting test surface (expected to fail for now)

    rst_vs_yaml!("lists/field_list/ok", test_case);
}

#[rstest]
#[case("definition_00")]
#[case("definition_01")]
#[case("definition_05")]
#[case("definition_06")]
#[case("definition_08")]
#[case("definition_09")]
#[case("definition_10")]
#[case("definition_11")]
#[case("definition_13")]
#[case("definition_14")]
#[case("definition_15")]
#[case("inline_markup")]
fn parse_definition_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/definition_list/ok", test_case);
}

#[rstest]
#[case("definition_02")]
#[case("definition_03")]
#[case("definition_04")]
#[case("definition_07")]
#[case("definition_12")]
fn rejects_definition_list_diagnostics(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/definition_list/err")
        .join(format!("{}.rst", test_case));
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    parse(&rst_contents).expect_err("expected definition-list diagnostic to fail parsing");
}

#[rstest]
#[case("empty_item_no_blank.rst")]
#[case("list_end_no_blank.rst")]
fn rejects_docutils_field_list_end(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/field_list/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListEndError { .. }));
}

#[rstest]
#[case("01_short_options")]
#[case("02_long_options")]
#[case("03_old_gnu_options")]
#[case("04_vms_dos_options")]
#[case("05_mixed_options")]
#[case("06_aliased_options")]
#[case("07_descriptions_aligned")]
#[case("08_descriptions_unaligned")]
// #[case("09_descriptions_next_line")]
// #[case("10_multiple_body_elements")]
#[case("11_empty_item_no_blank")]
// #[case("12_argument_delimiters")]
// #[case("13_edge_cases")] // TODO: Separate into errors and warnings (-> linter)
// #[case("14_complex_arguments")]
// TODO: Lexer currently panics on the Unicode arrow in this paragraph.
// #[case("15_incorrect_syntax")]
fn parse_option_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/option_list/ok", test_case)
}
