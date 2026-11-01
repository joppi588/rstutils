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
#[case("citations_00")]
#[case("citations_01")]
#[case("citations_02")]
#[case("citations_03")]
#[case("citations_04")]
#[case("citations_05")]
#[case("citations_07")]
// #[case("citations_08")] //TODO: #444
fn ok_citations(#[case] test_case: &str) {
    rst_vs_yaml!("citations", test_case);
}

#[rstest]
#[case("citations_06", PE::UnexpectedToken)]
fn err_citations(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("citations", test_case, expected_error);
}
