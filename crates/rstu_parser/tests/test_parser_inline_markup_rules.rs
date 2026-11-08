// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::markup_recognition_rules("markup_recognition_rules_00")]
#[case::markup_recognition_rules("markup_recognition_rules_01")]
#[case::markup_recognition_rules("markup_recognition_rules_02")]
#[case::markup_recognition_rules("markup_recognition_rules_03")]
#[case::markup_recognition_rules("markup_recognition_rules_04")]
#[case::markup_recognition_rules("markup_recognition_rules_05")]
#[case::markup_recognition_rules("markup_recognition_rules_06")]
#[case::markup_recognition_rules("markup_recognition_rules_07")]
#[ignore]
fn ok_inline_markup_rules(#[case] test_case: &str) {
    rst_vs_yaml!("inline", test_case)
}
