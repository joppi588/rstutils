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
#[case::o00("outdenting_00")]
#[case::o01("outdenting_01")]
#[case::o02("outdenting_02")]
fn ok_outdenting(#[case] test_case: &str) {
    rst_vs_yaml!("outdenting/ok", test_case);
}

#[rstest]
#[case::o03("outdenting_03", PE::MissingFinalBlankLineError)]
fn err_outdenting(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("outdenting/err", test_case, expected_error);
}
