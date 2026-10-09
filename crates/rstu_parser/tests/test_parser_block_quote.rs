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
#[case("block_quote_00")]
#[case("block_quote_01")]
// TODO: Nested block quotes currently do not terminate parsing.
// #[case("block_quote_04")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_05")]
// TODO: Unicode em-dash currently panics in the lexer; attribution is also unsupported.
// #[case("block_quote_06")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_07")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_08")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_09")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_10")]
// TODO: Block quote and definition-list interaction is not implemented.
// #[case("block_quote_11")]
// TODO: Attribution nodes are not implemented.
// #[case("block_quote_12")]
fn parse_block_quote(#[case] test_case: &str) {
    rst_vs_yaml!("block_quotes/ok", test_case);
}

#[rstest]
#[case("block_quote_02.rst")]
#[case("block_quote_03.rst")]
fn rejects_docutils_block_quote_diagnostics(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("block_quotes/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    parse(&rst_contents).expect_err("expected block-quote diagnostic to fail parsing");
}
