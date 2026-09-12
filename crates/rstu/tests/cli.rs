// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::process::Command;

fn run_rstu(args: &[&str]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_rstu"))
        .args(args)
        .output()
        .expect("failed to run rstu binary");

    assert!(
        output.status.success(),
        "command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    String::from_utf8(output.stdout)
        .expect("stdout should be valid UTF-8")
        .trim()
        .to_string()
}

#[test]
fn check_prints_stub_with_file_option_value() {
    let stdout = run_rstu(&["check", "sample.rst"]);
    assert_eq!(stdout, "Subcommand check, option file=sample.rst");
}

#[test]
fn format_without_output_prints_none() {
    let stdout = run_rstu(&["format", "sample.rst"]);
    assert_eq!(
        stdout,
        "Subcommand format, option file=sample.rst, output=none"
    );
}

#[test]
fn format_with_json_output_prints_option_value() {
    let stdout = run_rstu(&["format", "sample.rst", "--output", "json"]);
    assert_eq!(
        stdout,
        "Subcommand format, option file=sample.rst, output=json"
    );
}

/// GIVEN an rst file containing a single paragraph
/// WHEN running `rstu parse <file>`
/// THEN the AST is written as JSON to stdout
#[test]
fn parse_prints_ast_as_json() {
    let file_path =
        std::env::temp_dir().join(format!("rstu-parse-test-{}.rst", std::process::id()));
    std::fs::write(&file_path, "Hello\n").expect("failed to write rst fixture");

    let stdout = run_rstu(&[
        "parse",
        file_path.to_str().expect("fixture path should be UTF-8"),
    ]);

    let _ = std::fs::remove_file(&file_path);
    assert_eq!(
        stdout,
        r#"{"children":[{"children":[{"attributes":{"text":"Hello\n"},"class":"PlainText"}],"class":"Paragraph"}],"class":"Document"}"#
    );
}
