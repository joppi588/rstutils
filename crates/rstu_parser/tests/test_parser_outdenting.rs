// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("outdenting_00")]
#[case("outdenting_01")]
#[case("outdenting_02")]
fn ok_outdenting(#[case] test_case: &str) {
    rst_vs_yaml!("outdenting/ok", test_case);
}

#[rstest]
#[case("outdenting_03")]
fn err_outdenting(#[case] test_case: &str) {
    rst_err!("outdenting/err", test_case);
}
