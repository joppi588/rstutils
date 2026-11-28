// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

// Nested parsing in the docutils parsers defines a directive supporting nested secion parsing.
// In rstu, these tests demonstrate the standard parser behavior.

#[rstest]
#[case::n00("nested_parsing_00")]
#[case::n01("nested_parsing_01")]
#[case::n02("nested_parsing_02")]
#[case::n03("nested_parsing_03")]
#[case::n04("nested_parsing_04")]
#[case::n05("nested_parsing_05")]
#[case::n06("nested_parsing_06")]
#[case::n07("nested_parsing_07")]
#[case::n08("nested_parsing_08")]
#[case::n09("nested_parsing_09")]
fn ok_nested_parsing(#[case] test_case: &str) {
    // These fixtures check section nesting inside directive bodies, so parse the bodies as rst.
    let mut config = rstu_parser::ParserConfig::default();
    config.rst_directives.extend(
        [
            "nested",
            "nested-section",
            "nested-current",
            "fresh",
            "fresh-current",
        ]
        .map(String::from),
    );
    rst_vs_yaml!("nested_parsing", test_case, &config);
}
