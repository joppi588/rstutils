// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstest::rstest;
use rstu_parser::parse;
use std::fs;
use std::path::Path;

#[path = "common/mod.rs"]
mod test_parser;
use test_parser::rst_vs_yaml;

#[rstest]
#[case("indented_00")]
#[case("indented_02")]
#[case("indented_03")]
#[case("indented_08")]
#[case("indented_09")]
#[case("indented_11")]
#[case("quoted_00")]
#[case("quoted_01")]
#[case("quoted_02")]
// TODO: Activate after escaping is implemented
//#[case("indented_06")]
//#[case("indented_07")]

fn parse_literal_block(#[case] test_case: &str) {
    rst_vs_yaml!("literal_blocks", test_case);
}

#[rstest]
#[case("indented_04")]
#[case("quoted_03")]
#[case("quoted_04")]
#[case("quoted_05")]
fn rejects_literal_block_errors(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/literal_blocks")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(rst_path).unwrap();

    assert!(
        parse(&rst_contents).is_err(),
        "expected {test_case} to fail"
    );
}

#[test]
fn accepts_consistent_quoted_literal_markers() {
    for marker in "!#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".chars() {
        let source = format!("A paragraph::\n\n{marker}first\n{marker}second\n\n");
        assert!(
            parse(&source).is_ok(),
            "marker {marker:?} should be accepted"
        );
    }
}

#[test]
fn rejects_invalid_or_inconsistent_quoted_literal_markers() {
    for source in [
        "A paragraph::\n\nx first\nx second\n\n",
        "A paragraph::\n\n# first\n> second\n\n",
    ] {
        assert!(parse(source).is_err(), "expected quoted marker rejection");
    }
}

// #[case("indented_05")] // Emits a warning for an unindented continuation.
// #[case("indented_10")] // Emits an informational possible-title warning.
// #[case("indented_12")] // Emits a warning when no literal block follows.
// #[case("indented_14")] // Emits a warning when the marker reaches EOF.
