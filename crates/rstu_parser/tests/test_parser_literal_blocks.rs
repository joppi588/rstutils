// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::parse;
use std::fs;
use std::path::Path;

mod test_parser;

#[rstest]
#[case("indented_00")]
#[case("indented_01")]
#[case("indented_02")]
#[case("indented_03")]
#[case("indented_08")]
#[case("indented_09")]
#[case("indented_11")]
#[case("quoted_00")]
#[case("quoted_01")]
#[case("quoted_02")]
fn parse_literal_block(#[case] test_case: &str) {
    rst_vs_yaml!("literal_blocks", test_case)
}

#[rstest]
#[case("indented_04")]
#[case("indented_06")]
#[case("indented_07")]
#[case("quoted_03")]
#[case("quoted_04")]
#[case("quoted_05")]
fn rejects_literal_block_errors(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/literal_blocks")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(rst_path).unwrap();

    assert!(
        parse(&rst_contents).is_err(),
        "expected {test_case} to fail"
    );
}

// #[case("indented_05")] // Emits a warning for an unindented continuation.
// #[case("indented_10")] // Emits an informational possible-title warning.
// #[case("indented_12")] // Emits a warning when no literal block follows.
// #[case("indented_14")] // Emits a warning when the marker reaches EOF.
