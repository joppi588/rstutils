// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::c00("comments_00")]
#[case::c01("comments_01")]
#[case::c02("comments_02")]
#[case::c03("comments_03")]
#[case::c04("comments_04")]
#[case::c05("comments_05")]
#[case::c06("comments_06")]
#[case::c07("comments_07")]
#[case::c08("comments_08")]
#[case::c09("comments_09")]
#[case::c10("comments_10")]
#[case::c11("comments_11")]
#[case::c12("comments_12")]
#[case::c13("comments_13")]
#[case::c14("comments_14")]
#[case::c15("comments_15")]
#[case::c16("comments_16")]
#[case::c17("comments_17")]
fn ok_comments(#[case] test_case: &str) {
    rst_vs_yaml!("comments", test_case);
}
