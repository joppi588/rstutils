// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parser_errors::ParserErrorKind as PE;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_err;
use test_parser::rst_vs_yaml;

#[rstest]
#[case::b00("basics_00")]
#[case::b01("basics_01")]
#[case::b02("basics_02")]
#[case::b03a("basics_03a")]
#[case::b04("basics_04")]
#[case::b05("basics_05")]
#[case::b06("basics_06")]
#[case::b08("basics_08")]
#[case::b09("basics_09")]
// #[case::b10("basics_10")]// TODO: #468 Escaped char case
// #[case::b11("basics_11")]
#[case::b12("basics_12")]
#[case::b14("basics_14")]
#[case::b15("basics_15")]
#[case::c00("code_00")]
#[case::c00("code_parsing_00")]
#[case::c01("code_parsing_01")]
#[case::c02("code_parsing_02")]
#[case::r00("references_00")]
#[case::r01("references_01")]
#[case::r02("references_02")]
#[case::r03("references_03")]
#[case::r04("references_04")]
#[case::u00("unknown_roles_00")]
#[case::u01("unknown_roles_01")]
#[case::u03("unknown_roles_03")]
#[case::u04("unknown_roles_04")]
fn ok_interpreted(#[case] test_case: &str) {
    rst_vs_yaml!("interpreted", test_case);
}

#[rstest]
#[case::b03a("basics_03b", PE::InlineMissingClosing)]
#[case::b07("basics_07", PE::InlineMissingClosing)]
#[case::b13("basics_13", PE::InlineMissingClosing)]
#[case::u02("unknown_roles_02", PE::RolePrefixAndPostfix)]
fn err_interpreted(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("interpreted", test_case, expected_error);
}
