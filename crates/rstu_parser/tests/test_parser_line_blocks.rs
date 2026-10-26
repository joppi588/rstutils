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
#[case::l00("line_block_00")]
#[case::l01("line_block_01")]
#[case::l02("line_block_02")]
#[case::l03("line_block_03")]
#[case::l04("line_block_04")]
#[case::l06("line_block_06")]
#[case::l07("line_block_07")]
#[case::l08("line_block_08")]
#[case::l09("line_block_09")]
#[case::l10("line_block_10")]
#[case::l11("line_block_11")]
#[case::l12("line_block_12")]
#[case::l15("line_block_15")]
#[case::l16("line_block_16")]
fn ok_line_block(#[case] test_case: &str) {
    rst_vs_yaml!("line_blocks", test_case);
}

#[rstest]
#[case::l05("line_block_05", PE::UnexpectedToken)]
#[case::l13("line_block_13", PE::UnexpectedToken)]
#[case::l14("line_block_14", PE::InlineMissingClosing)]
fn err_line_block(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("line_blocks", test_case, expected_error);
}
