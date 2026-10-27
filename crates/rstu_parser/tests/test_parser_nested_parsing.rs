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
#[case("nested_parsing_01")]
#[case("nested_parsing_02")]
#[case("nested_parsing_03")]
#[case("nested_parsing_04")]
#[case("nested_parsing_09")]
fn ok_nested_parsing(#[case] test_case: &str) {
    rst_vs_yaml!("nested_parsing", test_case);
}

#[rstest]
#[case("nested_parsing_00", PE::UnexpectedToken)]
#[case("nested_parsing_05", PE::UnexpectedToken)]
#[case("nested_parsing_06", PE::UnexpectedToken)]
#[case("nested_parsing_07", PE::UnexpectedToken)]
#[case("nested_parsing_08", PE::UnexpectedToken)]
fn err_nested_parsing(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("nested_parsing", test_case, expected_error);
}
