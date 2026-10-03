// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("inline_variants")]
#[case("strong")]
fn parse_inline(#[case] test_case: &str) {
    rst_vs_yaml!("inline/ok", test_case)
}
