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
#[case::b03("basics_03")]
#[case::b04("basics_04")]
#[case::b05("basics_05")]
#[case::b06("basics_06")]
#[case::b08("basics_08")]
#[case::b09("basics_09")]
#[case::b10("basics_10")]
#[case::b11("basics_11")]
#[case::b12("basics_12")]
#[case::b14("basics_14")]
#[case::b15("basics_15")]
#[case::c00("code_00")]
#[case::c00("code_parsing_00")]
#[case::c01("code_parsing_01")]
#[case::c02("code_parsing_02")]
#[case::r00("references_00")]
#[case::r02("references_02")]
#[case::r04("references_04")]
#[ignore]
fn ok_interpreted(#[case] test_case: &str) {
    rst_vs_yaml!("interpreted", test_case);
}

#[rstest]
#[case::b07("basics_07", PE::UnexpectedToken)]
#[case::b13("basics_13", PE::UnexpectedToken)]
#[case::r01("references_01", PE::UnexpectedToken)]
#[case::r03("references_03", PE::UnexpectedToken)]
#[case::u00("unknown_roles_00", PE::UnexpectedToken)]
#[case::u01("unknown_roles_01", PE::UnexpectedToken)]
#[case::u02("unknown_roles_02", PE::UnexpectedToken)]
#[case::u03("unknown_roles_03", PE::UnexpectedToken)]
#[case::u04("unknown_roles_04", PE::UnexpectedToken)]
#[ignore]
fn err_interpreted(#[case] test_case: &str, #[case] expected_error: PE) {
    rst_err!("interpreted", test_case, expected_error);
}
