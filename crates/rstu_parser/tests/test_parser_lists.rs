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
#[case("bullet_00")]
#[case("bullet_01")]
#[case("bullet_02")]
#[case("bullet_03")]
#[case("bullet_04")]
#[case("bullet_07")]
#[case("bullet_09")]
#[case("bullet_list")]
#[case("compact_bullet_list")]
#[case("nested_bullet_list")]
fn ok_bullet_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/bullet_list/ok", test_case)
}

#[rstest]
#[case("bullet_05", PE::ListStyleError)]
fn err_bullet_list_style(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/bullet_list/err", test_case, expected_error);
}

#[rstest]
#[case("bullet_06", PE::ListEndError)]
#[case("bullet_08", PE::ListEndError)]
fn err_bullet_list_end(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/bullet_list/err", test_case, expected_error);
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
    rst_vs_yaml!("lists/enumerated_list/ok", test_case)
}

#[rstest]
#[case("unindented_enumerated_continuation", PE::ListEndError)]
fn err_enumerated_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/enumerated_list/err", test_case, expected_error);
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
#[case("complex_field_bodies")]
// TODO: NOT IMPLEMENTED:
// #[case("inline_markup_in_name")]
// #[case("bad_inline_markup")]
// #[case("edge_cases")]
// #[case("embedded_colons_comment_split")]
// #[case("embedded_colons_interpreted_text")]

fn ok_field_list(#[case] test_case: &str) {
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
fn ok_definition_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/definition_list/ok", test_case);
}

#[rstest]
#[case("definition_02", PE::MissingFinalBlankLineError)]
#[case("definition_03", PE::LiteralBlockMissingBlankLine)]
#[case("definition_04", PE::UnexpectedIndentError)]
#[case("definition_07", PE::MissingFinalBlankLineError)]
#[case("definition_12", PE::InlineMissingClosing)]
fn err_definition_list(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/definition_list/err", test_case, expected_error);
}

#[rstest]
#[case("empty_item_no_blank", PE::ListEndError)]
#[case("list_end_no_blank", PE::ListEndError)]
fn err_field_list_end(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("lists/field_list/err", test_case, expected_error);
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
#[case("09_descriptions_next_line")]
#[case("10_multiple_body_elements")]
#[case("11_empty_item_no_blank")]
#[case("12_argument_delimiters")]
// #[case("13_edge_cases")] // TODO: Separate into errors and warnings (-> linter)
// #[case("14_complex_arguments")]
// TODO: Lexer currently panics on the Unicode arrow in this paragraph.
// #[case("15_incorrect_syntax")]
fn ok_option_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/option_list/ok", test_case)
}
