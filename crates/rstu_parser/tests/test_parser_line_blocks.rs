// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("line_block_00")]
#[case("line_block_01")]

fn parse_line_block(#[case] test_case: &str) {
    rst_vs_yaml!("line_blocks/ok", test_case);
}
