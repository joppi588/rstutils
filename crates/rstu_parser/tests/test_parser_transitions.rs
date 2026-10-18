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
#[case("transitions_00")]
#[case("transitions_01")]
#[case("transitions_02")]
#[case("transitions_04")]
#[case("transitions_05")]
#[case("transitions_06")]
#[case("transitions_07")]
#[case("transitions_08")]
#[case("transitions_09")]
#[case("transitions_10")]
#[case("transitions_11")]
#[case("transitions_12")]
#[ignore = "transition parsing and expected YAML fixtures are not implemented yet"]
fn parse_transitions(#[case] test_case: &str) {
    rst_vs_yaml!("transitions/ok", test_case)
}

#[rstest]
#[case("transitions_03")]
fn rejects_docutils_transition_diagnostics(#[case] test_case: &str) {
    let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/transitions/err")
        .join(format!("{test_case}.rst"));
    let rst_contents = fs::read_to_string(rst_path).unwrap();

    parse(&rst_contents).expect_err("expected transition diagnostic to fail parsing");
}
