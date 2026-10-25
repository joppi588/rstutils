// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use rstu_parser::parser_errors::ParserErrorKind as PE;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("block_quote_00")]
#[case("block_quote_01")]
#[case("block_quote_04")]
#[case("block_quote_05")]
#[case("block_quote_06")]
#[case("block_quote_07")]
#[case("block_quote_08")]
#[case("block_quote_09")]
#[case("block_quote_10")]
#[case("block_quote_12")]
fn ok_block_quote(#[case] test_case: &str) {
    rst_vs_yaml!("block_quotes/ok", test_case);
}

#[rstest]
#[case("block_quote_02", PE::UnexpectedIndentError)]
#[case("block_quote_03", PE::MissingFinalBlankLineError)]
#[case("block_quote_11", PE::UnexpectedToken)] // TODO: This shall trigger an unexpected indent error
#[case("block_quote_13", PE::MissingFinalBlankLineError)] // TODO: This shall trigger an unexpected indent error
fn err_block_quote_diagnostics(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("block_quotes/err", test_case, expected_error);
}
