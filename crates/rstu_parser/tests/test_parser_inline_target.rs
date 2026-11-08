// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::inline_targets("inline_targets_00")]
#[case::inline_targets("inline_targets_01")]
#[case::inline_targets("inline_targets_02")]
#[case::inline_targets("inline_targets_03")]
#[ignore]
fn ok_inline_target(#[case] test_case: &str) {
    rst_vs_yaml!("inline/inline_internal_target", test_case)
}

// TODO: Err cases
// #[case::inline_targets("inline_targets_04")]
// #[case::inline_targets("inline_targets_05")]
