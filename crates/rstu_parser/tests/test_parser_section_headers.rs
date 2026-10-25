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
#[case("section_headers_00")]
#[case("section_headers_01")]
#[case("section_headers_02")]
#[case("section_headers_05")]
#[case("section_headers_06")]
#[case("section_headers_07")]
#[case("section_headers_13")]
#[case("section_headers_15")]
#[case("section_headers_16")]
#[case("section_headers_17")]
#[case("section_headers_20")]
#[case("section_headers_21")]
#[case("section_headers_23")]
#[case("section_headers_31")]
#[case("section_headers_34")]
#[case("section_headers_35")]
#[case("section_headers_36")]
fn ok_section_headers(#[case] test_case: &str) {
    rst_vs_yaml!("section_headers/ok", test_case);
}

#[rstest]
#[case("section_headers_03", PE::UnexpectedToken)]
#[case("section_headers_04", PE::UnexpectedToken)]
#[case("section_headers_08", PE::SectionTitleMissingClosingAfterOpening)]
#[case("section_headers_09", PE::UnexpectedToken)]
#[case("section_headers_10", PE::UnexpectedToken)]
#[case("section_headers_11", PE::UnexpectedToken)]
#[case("section_headers_12", PE::UnexpectedToken)]
#[case("section_headers_14", PE::UnexpectedToken)]
#[case("section_headers_18", PE::UnexpectedToken)]
#[case("section_headers_19", PE::UnexpectedToken)]
#[case("section_headers_22", PE::UnexpectedToken)]
#[case("section_headers_24", PE::UnexpectedToken)]
#[case("section_headers_25", PE::UnexpectedToken)]
#[case("section_headers_26", PE::UnexpectedToken)]
#[case("section_headers_27", PE::UnexpectedToken)]
#[case("section_headers_28", PE::UnexpectedToken)]
#[case("section_headers_29", PE::UnexpectedToken)]
#[case("section_headers_30", PE::UnexpectedToken)]
#[case("section_headers_32", PE::UnexpectedToken)]
#[case("section_headers_33", PE::UnexpectedToken)]
fn err_section_headers(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("section_headers/err", test_case, expected_error);
}
