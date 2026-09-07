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
#[case("directives", "ok_note_simple")]
#[case("directives", "ok_comment")]
#[case("directives", "ok_image_numeric_options")]
#[case("directives", "ok_image_options_and_content")]
fn parses_directives_and_matches_yaml_fixture(#[case] directory: &str, #[case] test_case: &str) {
    rst_vs_yaml!(directory, test_case)
}
