// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("comments_00")]
#[case("comments_01")]
#[case("comments_02")]
#[case("comments_03")]
#[case("comments_04")]
#[case("comments_05")]
#[case("comments_06")]
#[case("comments_07")]
#[case("comments_08")]
#[case("comments_09")]
#[case("comments_10")]
#[case("comments_11")]
#[case("comments_12")]
#[case("comments_13")]
#[case("comments_14")]
#[case("comments_15")]
#[case("comments_16")]
#[case("comments_17")]
fn ok_comments(#[case] test_case: &str) {
    rst_vs_yaml!("comments", test_case);
}
