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
#[case("doctest_block_00")]
#[case("doctest_block_01")]
#[case("doctest_block_02")]

fn ok_doctest_blocks(#[case] test_case: &str) {
    rst_vs_yaml!("doctest_blocks/ok", test_case);
}

#[rstest]
#[case("text >>> more\n")]
#[case("text\n>>> not a doctest\n")]
fn ok_doctest_marker_without_blank_line_is_paragraph_text(#[case] source: &str) {
    let yaml = rstu_ast::AstNode::to_yaml(&parse(source).unwrap()).unwrap();
    assert!(!yaml.contains("DoctestBlock"), "{yaml}");
}

#[test]
fn ok_doctest_block_at_eof_without_blank_line() {
    let yaml = rstu_ast::AstNode::to_yaml(&parse("Text.\n\n>>> 1 + 1\n2").unwrap()).unwrap();
    assert!(yaml.contains("DoctestBlock"), "{yaml}");
}

#[rstest]
#[case("doctest_block_03")]
fn err_doctest_block(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/doctest_blocks/err")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(rst_path).unwrap();

    assert!(
        parse(&rst_contents).is_err(),
        "expected {test_case} to fail"
    );
}
