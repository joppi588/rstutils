// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("01_short_options")]
#[case("02_long_options")]
// #[case("03_old_gnu_options")]
// #[case("04_vms_dos_options")]
// #[case("05_mixed_options")]
// #[case("06_aliased_options")]
// #[case("07_descriptions_aligned")]
// #[case("08_descriptions_unaligned")]
// #[case("09_descriptions_next_line")]
// #[case("10_multiple_body_elements")]
#[case("11_empty_item_no_blank")]
// #[case("12_argument_delimiters")]
// #[case("13_edge_cases")] // TODO: Separate into errors and warnings (-> linter)
// #[case("14_complex_arguments")]
// TODO: Lexer currently panics on the Unicode arrow in this paragraph.
// #[case("15_incorrect_syntax")]
fn parse_option_list(#[case] test_case: &str) {
    rst_vs_yaml!("lists/option_list/ok", test_case)
}
