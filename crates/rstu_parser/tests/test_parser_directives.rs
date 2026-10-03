// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("figure")]
#[case("image_numeric_options")]
#[case("image_options_and_content")]
#[case("nested_directive")]
#[case("note_compound_block")]
#[case("note_simple")]
#[case("note_then_paragraph")]
fn parse_directive(#[case] test_case: &str) {
    rst_vs_yaml!("directives/ok", test_case)
}
