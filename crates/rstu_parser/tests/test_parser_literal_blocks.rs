// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parse;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("indented_00")]
#[case("indented_01")]
#[case("indented_02")]
#[case("indented_05")]
#[case("indented_06")]
#[case("indented_07")]
#[case("indented_08")]
#[case("indented_09")]
#[case("quoted_00")]
#[case("quoted_01")]
#[case("quoted_02")]
fn ok_literal_block(#[case] test_case: &str) {
    rst_vs_yaml!("literal_blocks/ok", test_case);
}

#[rstest]
#[case("indented_03")]
#[case("indented_04")]
#[case("indented_10")]
#[case("indented_12")]
#[case("quoted_03")]
#[case("quoted_04")]
#[case("quoted_05")]
#[case("quoted_block_01")]
#[case("quoted_block_02")]
fn err_literal_block(#[case] test_case: &str) {
    rst_err!("literal_blocks/err", test_case);
}

#[test]
fn ok_consistent_quoted_literal_markers() {
    for marker in "!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars() {
        let source = format!("A paragraph::\n\n{marker}first\n{marker}second\n\n");
        assert!(
            parse(&source).is_ok(),
            "marker {marker:?} should be accepted"
        );
    }
}
