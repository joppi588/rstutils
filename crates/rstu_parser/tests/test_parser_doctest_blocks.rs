// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parse;
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("doctest_blocks_00")]
#[case("doctest_blocks_01")]
#[case("doctest_blocks_02")]

fn parse_doctest_blocks(#[case] test_case: &str) {
    rst_vs_yaml!("doctest_blocks/ok", test_case);
}
