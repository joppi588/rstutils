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
#[case::b00("block_quote_00")]
#[case::b01("block_quote_01")]
#[case::b04("block_quote_04")]
#[case::b05("block_quote_05")]
#[case::b06("block_quote_06")]
#[case::b07("block_quote_07")]
#[case::b08("block_quote_08")]
#[case::b09("block_quote_09")]
#[case::b10("block_quote_10")]
#[case::b12("block_quote_12")]
fn ok_block_quote(#[case] test_case: &str) {
    rst_vs_yaml!("block_quotes", test_case);
}

#[rstest]
#[case::b02("block_quote_02", PE::UnexpectedIndentError)]
#[case::b03("block_quote_03", PE::MissingFinalBlankLineError)]
#[case::b11("block_quote_11", PE::UnexpectedToken)] // TODO: This shall trigger an unexpected indent error
#[case::b13("block_quote_13", PE::MissingFinalBlankLineError)] // TODO: This shall trigger an unexpected indent error
fn err_block_quote(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("block_quotes", test_case, expected_error);
}
