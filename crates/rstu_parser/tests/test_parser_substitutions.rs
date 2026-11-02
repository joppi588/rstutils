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
#[case("substitution_definitions_00")]
#[case("substitution_definitions_01")]
#[case("substitution_definitions_02")]
#[case("substitution_definitions_03")]
#[case("substitution_definitions_04")]
#[case("substitution_definitions_06")]
#[case("substitution_definitions_07")]
#[case("substitution_definitions_08")]
#[ignore]
fn ok_substitutions(#[case] test_case: &str) {
    rst_vs_yaml!("substitutions", test_case);
}

#[rstest]
#[case::s05("substitution_definitions_05", PE::UnexpectedToken)]
#[case::s09("substitution_definitions_09", PE::UnexpectedToken)]
#[case::s10("substitution_definitions_10", PE::UnexpectedToken)]
#[case::s11("substitution_definitions_11", PE::UnexpectedToken)]
#[ignore]
fn err_substitutions(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("substitutions", test_case, expected_error);
}
