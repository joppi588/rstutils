// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_ast::AstNode;
use rstu_parser::parse;
mod test_parser;
use std::fs;
use std::path::Path;

#[rstest]
#[case("inline", "ok_strong.rst", "ok_strong.yaml")]
#[case("inline", "ok_inline_variants.rst", "ok_inline_variants.yaml")]
fn parse_inline(#[case] directory: &str, #[case] rst_filename: &str, #[case] yaml_filename: &str) {
    rst_vs_yaml!(directory, rst_filename, yaml_filename)
}
