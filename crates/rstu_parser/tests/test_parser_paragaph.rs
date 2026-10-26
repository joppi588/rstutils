// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;
#[rstest]
#[case("paragraph_w_bullet")]
#[case::p00("paragraphs_00")]
#[case::p01("paragraphs_01")]
#[case::p02("paragraphs_02")]
#[case::p03("paragraphs_03")]
// TODO: A. Einstein case, tbc. no error in docutils.
// #[case::p04("paragraphs_04")]
#[case::p05("paragraphs_05")]
fn ok_paragraph(#[case] test_case: &str) {
    rst_vs_yaml!("paragraphs", test_case)
}
