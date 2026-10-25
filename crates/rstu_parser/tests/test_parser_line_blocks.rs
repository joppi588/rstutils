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
#[case("line_block_00")]
#[case("line_block_01")]
#[case("line_block_02")]
#[case("line_block_03")]
#[case("line_block_04")]
#[case("line_block_06")]
#[case("line_block_07")]
#[case("line_block_08")]
#[case("line_block_09")]
#[case("line_block_10")]
#[case("line_block_11")]
#[case("line_block_12")]
#[case("line_block_15")]
#[case("line_block_16")]
fn ok_line_block(#[case] test_case: &str) {
    rst_vs_yaml!("line_blocks/ok", test_case);
}

#[rstest]
#[case("line_block_05", PE::UnexpectedToken)]
#[case("line_block_13", PE::UnexpectedToken)]
#[case("line_block_14", PE::InlineMissingClosing)]
fn err_line_block(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("line_blocks/err", test_case, expected_error);
}
