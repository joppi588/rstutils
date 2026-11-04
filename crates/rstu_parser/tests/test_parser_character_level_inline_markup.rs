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
#[case::e01("emphasis_01")]
#[case::s00("strong_00")]
#[case::s01("strong_01")]
#[case::l00("literal_00")]
#[case::l02("literal_02")]
#[case::r00("references_00")]
#[case::e00("embedded_uris_00")]
#[case::i00("inline_targets_00")]
#[case::f00("footnote_reference_00")]
#[case::c00("citation_reference_00")]
#[case::s00("substitution_references_00")]
#[case::m00("markup_recognition_rules_00")]
#[case::m01("markup_recognition_rules_01")]
#[case::m02("markup_recognition_rules_02")]
#[case::m03("markup_recognition_rules_03")]
#[case::m04("markup_recognition_rules_04")]
#[case::m05("markup_recognition_rules_05")]
#[ignore]
fn ok_character_level_inline_markup(#[case] test_case: &str) {
    rst_vs_yaml!("character_level_inline_markup", test_case);
}

#[rstest]
#[case::e00("emphasis_00", PE::UnexpectedToken)]
#[case::l01("literal_01", PE::UnexpectedToken)]
#[case::s00("standalone_hyperlink_00", PE::UnexpectedToken)]
#[ignore]
fn err_character_level_inline_markup(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("character_level_inline_markup", test_case, expected_error);
}
