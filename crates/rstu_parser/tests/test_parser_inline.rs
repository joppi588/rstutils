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
#[case("inline", "ok_strong")]
#[case("inline", "ok_inline_variants")]
fn parse_inline(#[case] directory: &str, #[case] filename_stem: &str) {
    rst_vs_yaml!(directory, filename_stem)
}
