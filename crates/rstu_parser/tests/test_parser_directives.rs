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
#[case("directives", "ok_note_simple.rst", "ok_note_simple.yaml")]
#[case("directives", "ok_comment.rst", "ok_comment.yaml")]
fn parses_directives_and_matches_yaml_fixture(
    #[case] directory: &str,
    #[case] rst_filename: &str,
    #[case] yaml_filename: &str,
) {
    rst_vs_yaml!(directory, rst_filename, yaml_filename)
}
