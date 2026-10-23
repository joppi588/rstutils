// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parser_errors::ParserError;
#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("sections_type1")]
#[case("sections_type2")]
fn ok_section(#[case] test_case: &str) {
    rst_vs_yaml!("sections/ok", test_case)
}

#[test]
fn err_section_missing_closing() {
    match rst_err!("sections/err", "sections_missing_closing") {
        ParserError::SectionTitleMissingClosingAfterOpening { .. } => {}
        other => panic!("expected SectionTitleMissingClosingAfterOpening error, got: {other:?}"),
    }
}

#[test]
fn err_section_unbalanced_style() {
    match rst_err!("sections/err", "sections_unbalanced_style") {
        ParserError::SectionTitleUnbalancedStyle {
            opening_style,
            closing_style,
            ..
        } => {
            assert_eq!(opening_style, "-");
            assert_eq!(closing_style, "=");
        }
        other => panic!("expected SectionTitleUnbalancedStyle error, got: {other:?}"),
    }
}
