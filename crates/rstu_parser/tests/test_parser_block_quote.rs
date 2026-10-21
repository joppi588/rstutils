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
#[case("outdenting_00")]
#[case("outdenting_01")]
#[case("outdenting_02")]
fn ok_outdenting(#[case] test_case: &str) {
    rst_vs_yaml!("block_quotes/ok", test_case);
}

#[rstest]
#[case("outdenting_03")]
fn err_outdenting(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("block_quotes/err")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    parse(&rst_contents).expect_err("expected block-quote diagnostic to fail parsing");
}
