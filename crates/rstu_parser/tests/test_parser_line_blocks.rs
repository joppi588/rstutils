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
fn parse_line_block(#[case] test_case: &str) {
    rst_vs_yaml!("line_blocks/ok", test_case);
}

#[rstest]
#[case("line_block_05")]
#[case("line_block_13")]
#[case("line_block_14")]
fn rejects_docutils_line_block_diagnostics(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/line_blocks/err")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(rst_path).unwrap();

    parse(&rst_contents).expect_err("expected line-block diagnostic to fail parsing");
}
