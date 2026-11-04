// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parser_errors::ParserErrorKind as PE;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;
#[rstest]
#[case::b00("bullet_00")]
#[case::b01("bullet_01")]
#[case::b02("bullet_02")]
#[case::b03("bullet_03")]
#[case::b04("bullet_04")]
#[case::b07("bullet_07")]
#[case::b09("bullet_09")]
#[case("bullet_list")]
#[case("compact_bullet_list")]
#[case("nested_bullet_list")]
fn ok_bullet_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/bullet_list", test_case)
}

#[rstest]
#[case::b05("bullet_05", PE::ListStyleError)]
#[case::b06("bullet_06", PE::ListEndError)]
#[case::b08("bullet_08", PE::ListEndError)]
fn err_bullet_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/bullet_list", test_case, expected_error);
}

#[rstest]
#[case::d00("definition_00")]
#[case::d01("definition_01")]
#[case::d05("definition_05")]
#[case::d06("definition_06")]
#[case::d08("definition_08")]
#[case::d09("definition_09")]
#[case::d10("definition_10")]
#[case::d11("definition_11")]
// #[case::d13("definition_13")] // TODO: #439
// #[case::d14("definition_14")] // TODO: #439
#[case::d15("definition_15")]
#[case("inline_markup")]
fn ok_definition_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/definition_list", test_case);
}

#[rstest]
#[case::d02("definition_02", PE::MissingFinalBlankLineError)]
#[case::d03("definition_03", PE::LiteralBlockMissingBlankLine)]
#[case::d04("definition_04", PE::UnexpectedIndentError)]
#[case::d07("definition_07", PE::MissingFinalBlankLineError)]
#[case::d12("definition_12", PE::InlineMissingClosing)]
fn err_definition_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/definition_list", test_case, expected_error);
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
fn ok_enumerated_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/enumerated_list", test_case)
}

#[rstest]
#[case("unindented_enumerated_continuation", PE::ListEndError)]
fn err_enumerated_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/enumerated_list", test_case, expected_error);
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
#[case::f01("01_bodies_next_line")]
#[case::f02("02_multiline_aligned")]
#[case::f03("03_multiline_not_lined_up")]
#[case::f04("04_multiple_arguments")]
#[case::f05("05_multiple_body_elements")]
#[case::f06("06_nested_one_line")]
#[case::f07("07_field_list")]
#[case::f08("08_oneliners_no_blank")]
#[case::f09("09_complex_field_bodies")]
// TODO: NOT IMPLEMENTED:
// #[case::inline_markup_in_name("inline_markup_in_name")]
// #[case::bad_inline_markup("bad_inline_markup")]
// #[case::edge_cases("edge_cases")]
// #[case::embedded_colons_comment_split("embedded_colons_comment_split")]
// #[case::embedded_colons_interpreted_text("embedded_colons_interpreted_text")]
// #[case::oneliners("oneliners")]
fn ok_field_list(#[case] test_case: &str) {
    // GIVEN field-list examples
    // WHEN we parse and compare them against YAML snapshots
    // THEN this covers the currently supported compatibility fixtures

    rst_vs_yaml!("lists/field_list", test_case);
}

#[rstest]
#[case::f10("10_empty_item_no_blank", PE::ListEndError)]
#[case::f11("11_list_end_no_blank", PE::ListEndError)]
fn err_field_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/field_list", test_case, expected_error);
}

#[rstest]
#[case::o01("01_short_options")]
#[case::o02("02_long_options")]
#[case::o03("03_old_gnu_options")]
#[case::o04("04_vms_dos_options")]
#[case::o05("05_mixed_options")]
#[case::o06("06_aliased_options")]
#[case::o07("07_descriptions_aligned")]
#[case::o08("08_descriptions_unaligned")]
#[case::o09("09_descriptions_next_line")]
#[case::o10("10_multiple_body_elements")]
#[case::o11("11_empty_item_no_blank")]
#[case::o12("12_argument_delimiters")]
// #[case::o13("13_edge_cases")] // TODO: Separate into errors and warnings (-> linter)
// #[case::o14("14_complex_arguments")]
// TODO: Lexer currently panics on the Unicode arrow in this paragraph.
// #[case::o15("15_incorrect_syntax")]
fn ok_option_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/option_list", test_case)
}
