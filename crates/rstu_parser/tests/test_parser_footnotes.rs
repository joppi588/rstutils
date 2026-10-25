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
#[case("auto_numbered_footnotes_00")]
#[case("auto_numbered_footnotes_01")]
#[case("auto_numbered_footnotes_02")]
#[case("auto_numbered_footnotes_03")]
#[case("auto_symbol_footnotes_00")]
#[case("footnotes_00")]
#[case("footnotes_01")]
#[case("footnotes_02")]
#[case("footnotes_03")]
#[case("footnotes_04")]
fn ok_footnotes(#[case] test_case: &str) {
    rst_vs_yaml!("footnotes/ok", test_case);
}

#[rstest]
#[case("footnotes_05", PE::UnexpectedToken)]
fn err_footnotes(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("footnotes/err", test_case, expected_error);
}
