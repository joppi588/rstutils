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
#[case::emphasis("emphasis_00")]
#[case::emphasis("emphasis_01")]
#[case::emphasis("emphasis_02")]
#[case::emphasis("emphasis_05")]
#[case::emphasis("emphasis_06")]
#[case::strong("strong_00")]
#[case::strong("strong_01")]
#[case::strong("strong_02")]
// #[case::strong("strong_04")] TODO: #439
fn ok_inline_markup(#[case] test_case: &str) {
    rst_vs_yaml!("inline/markup", test_case)
}

#[rstest]
#[case::l00("literal_00")]
#[case::l01("literal_01")]
#[case::l02("literal_02")]
// #[case::l03("literal_03")] // TODO: #464
#[case::l04("literal_04")]
#[case::l05("literal_05")]
#[case::l06("literal_06")]
#[case::l08("literal_08")]
#[case::l10("literal_10")]
#[case::l11("literal_11")]
#[case::l12("literal_12")]
#[case::l13("literal_13")]
fn ok_literal(#[case] test_case: &str) {
    rst_vs_yaml!("inline/markup", test_case)
}

#[rstest]
#[case::l07("literal_07", PE::InlineMissingClosing)]
#[case::l09("literal_09", PE::InlineMissingClosing)]
fn err_literal(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("inline/markup", test_case, expected_error);
}

// TODO: Err cases
// #[case::emphasis("emphasis_03")]
// #[case::emphasis("emphasis_04")]
// #[case::strong("strong_03")]
// #[case::strong("strong_05")]
