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
#[case::t00("transitions_00")]
#[case::t01("transitions_01")]
#[case::t02("transitions_02")]
#[case::t04("transitions_04")]
#[case::t05("transitions_05")]
#[case::t06("transitions_06")]
#[case::t07("transitions_07")]
#[case::t08("transitions_08")]
#[case::t09("transitions_09")]
#[case::t10("transitions_10")]
#[case::t11("transitions_11")]
#[case::t12("transitions_12")]
fn ok_transitions(#[case] test_case: &str) {
    rst_vs_yaml!("transitions/ok", test_case)
}

// TODO: Implement
#[rstest]
#[case::t03("transitions_03", PE::UnexpectedToken)]
#[ignore = "Docutils system-message diagnostics for indented separators are not implemented"]
fn err_transition_diagnostics(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("transitions/err", test_case, expected_error);
}
