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
#[case("transitions_00")]
#[case("transitions_01")]
#[case("transitions_02")]
#[case("transitions_04")]
#[case("transitions_05")]
#[case("transitions_06")]
#[case("transitions_07")]
#[case("transitions_08")]
#[case("transitions_09")]
#[case("transitions_10")]
#[case("transitions_11")]
#[case("transitions_12")]
fn ok_transitions(#[case] test_case: &str) {
    rst_vs_yaml!("transitions/ok", test_case)
}

// TODO: Implement
#[rstest]
#[case("transitions_03", PE::UnexpectedToken)]
#[ignore = "Docutils system-message diagnostics for indented separators are not implemented"]
fn err_transition(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("transitions/err", test_case, expected_error);
}
