// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::s00("substitution_definitions_00")]
#[case::s01("substitution_definitions_01")]
#[case::s02("substitution_definitions_02")]
#[case::s03("substitution_definitions_03")]
#[case::s04("substitution_definitions_04")]
#[case::s05("substitution_definitions_05")]
#[case::s06("substitution_definitions_06")]
#[case::s07("substitution_definitions_07")]
#[case::s08("substitution_definitions_08")]
#[case::s09("substitution_definitions_09")]
#[case::s10("substitution_definitions_10")]
#[case::s11("substitution_definitions_11")]
fn ok_substitutions(#[case] test_case: &str) {
    rst_vs_yaml!("substitutions", test_case);
}
