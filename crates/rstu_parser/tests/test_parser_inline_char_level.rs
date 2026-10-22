// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::{parse, parser_errors::ParserError};
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;
#[rstest]
#[case("emphasis_01")]
#[case("strong_00")]
#[case("strong_01")]
#[case("literal_00")]
#[case("literal_02")]
#[case("references_00")]
#[case("embedded_uris_00")]
#[case("inline_targets_00")]
#[case("footnote_reference_00")]
#[case("citation_reference_00")]
#[case("substitution_references_00")]
#[case("markup_recognition_rules_00")]
#[case("markup_recognition_rules_01")]
#[case("markup_recognition_rules_02")]
#[case("markup_recognition_rules_03")]
#[case("markup_recognition_rules_04")]
#[case("markup_recognition_rules_05")]
#[ignore]
fn ok_char_level_inline(#[case] test_case: &str) {
    rst_vs_yaml!("lists/bullet_list/ok", test_case)
}

#[rstest]
#[case("emphasis_00")]
#[case("literal_01")]
#[case("standalone_hyperlink_00")]
#[ignore]
fn err_char_level_inline(#[case] rst_filename: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join("lists/bullet_list/err")
        .join(rst_filename);
    let rst_contents = fs::read_to_string(&rst_path)
        .unwrap_or_else(|_| panic!("failed to read fixture file: {}", rst_path.display()));

    let err = parse(&rst_contents).unwrap_err();

    assert!(matches!(err, ParserError::ListStyleError { .. }));
}
