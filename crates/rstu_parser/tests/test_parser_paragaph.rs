// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;
#[rstest]
#[case("paragraph_w_bullet")]
#[case("paragraphs_01")]
#[case("paragraphs_02")]
#[case("paragraphs_03")]
// TODO: A. Einstein case, tbc. no error in docutils.
// #[case("paragraphs_04")]
#[case("paragraphs_05")]
fn parse_paragraph(#[case] test_case: &str) {
    rst_vs_yaml!("paragraphs/ok", test_case)
}
