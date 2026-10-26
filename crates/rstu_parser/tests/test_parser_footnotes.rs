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
#[case::a00("auto_numbered_footnotes_00")]
#[case::a01("auto_numbered_footnotes_01")]
#[case::a02("auto_numbered_footnotes_02")]
#[case::a03("auto_numbered_footnotes_03")]
#[case::a00("auto_symbol_footnotes_00")]
#[case::f00("footnotes_00")]
#[case::f01("footnotes_01")]
#[case::f02("footnotes_02")]
#[case::f03("footnotes_03")]
#[case::f04("footnotes_04")]
fn ok_footnotes(#[case] test_case: &str) {
    rst_vs_yaml!("footnotes/ok", test_case);
}

#[rstest]
#[case::f05("footnotes_05", PE::UnexpectedToken)]
fn err_footnotes(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("footnotes/err", test_case, expected_error);
}
