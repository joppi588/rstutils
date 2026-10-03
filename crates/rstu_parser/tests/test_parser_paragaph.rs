// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;
#[rstest]
#[case("paragraph_w_bullet")]
// #[case("bullet_09")]
fn parse_paragraph(#[case] test_case: &str) {
    rst_vs_yaml!("paragraph/ok", test_case)
}
