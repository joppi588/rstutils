// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("block_quote_00")]
fn parse_block_quote(#[case] test_case: &str) {
    rst_vs_yaml!("block_quotes/ok", test_case);
}
