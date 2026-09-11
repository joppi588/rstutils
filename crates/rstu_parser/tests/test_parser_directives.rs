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
#[case("ok_note_simple")]
#[case("ok_note_then_paragraph")]
#[case("ok_comment")]
#[case("ok_image_numeric_options")]
#[case("ok_image_options_and_content")]
fn parse_directive(#[case] test_case: &str) {
    rst_vs_yaml!("directives", test_case)
}
