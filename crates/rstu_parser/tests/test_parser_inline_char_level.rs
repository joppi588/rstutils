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
#[case::c00("citation_reference_00")] // FIX
#[case::e00("embedded_uris_00")]
// #[case::e01("emphasis_01")]
// #[case::f00("footnote_reference_00")]
// #[case::i00("inline_targets_00")]
// #[case::l00("literal_00")]
// #[case::l02("literal_02")]
#[case::m00("markup_recognition_rules_00")]
#[case::m01("markup_recognition_rules_01")] // FIX
#[case::m02("markup_recognition_rules_02")] // FIX
#[case::m03("markup_recognition_rules_03")]
#[case::m04("markup_recognition_rules_04")]
// #[case::r00("references_00")]
// #[case::s00("strong_00")]
// #[case::s01("strong_01")]
// #[case::s00("substitution_references_00")]
#[case::m05("markup_recognition_rules_05")]
fn ok_char_level_inline(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup_char_level", test_case)
}

#[rstest]
#[case::e00("emphasis_00", PE::InlineMissingClosing)]
#[case::l01("literal_01", PE::InlineMissingClosing)]
fn err_char_level_inline(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("inline_markup_char_level", test_case, expected_error);
}
