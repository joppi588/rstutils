// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;
#[rstest]
// #[case("citation_reference_00")]
// #[case("embedded_uris_00")]
// #[case("emphasis_01")]
// #[case("footnote_reference_00")]
// #[case("inline_targets_00")]
// #[case("literal_00")]
// #[case("literal_02")]
// #[case("markup_recognition_rules_00")]
// #[case("markup_recognition_rules_01")]
// #[case("markup_recognition_rules_02")]
// #[case("markup_recognition_rules_03")]
// #[case("markup_recognition_rules_04")]
// #[case("references_00")]
// #[case("strong_00")]
// #[case("strong_01")]
// #[case("substitution_references_00")]
#[case("markup_recognition_rules_05")]
#[ignore]
fn ok_char_level_inline(#[case] test_case: &str) {
    rst_vs_yaml!("inline_markup_char_level/ok", test_case)
}

#[rstest]
#[case("emphasis_00")]
#[case("literal_01")]
#[case("standalone_hyperlink_00")]
#[ignore]
fn err_char_level_inline(#[case] test_case: &str) {
    rst_err!("inline_markup_char_level/err", test_case);
}
