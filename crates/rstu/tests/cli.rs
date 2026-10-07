// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::path::PathBuf;
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

fn write_rst_fixture(name: &str, contents: &str) -> PathBuf {
    let file_path = std::env::temp_dir().join(format!("rstu-{name}-{}.rst", std::process::id()));
    std::fs::write(&file_path, contents).expect("failed to write rst fixture");
    file_path
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
/// WHEN running `rstu convert <file>`
/// THEN the AST is written as JSON to stdout
#[test]
fn convert_prints_ast_as_json_by_default() {
    let file_path = write_rst_fixture("convert-json", "Hello\n");

    let stdout = run_rstu(&[
        "convert",
        file_path.to_str().expect("fixture path should be UTF-8"),
    ]);

    let _ = std::fs::remove_file(&file_path);
    assert_eq!(
        stdout,
        r#"{"class":"Document","children":[{"class":"Paragraph","children":[{"class":"PlainText","attributes":{"text":"Hello\n"}}]}]}"#
    );
}

#[test]
fn convert_prints_ast_as_yaml_when_requested() {
    let file_path = write_rst_fixture("convert-yaml", "Hello\n");

    let stdout = run_rstu(&[
        "convert",
        file_path.to_str().expect("fixture path should be UTF-8"),
        "-o",
        "yaml",
    ]);

    let _ = std::fs::remove_file(&file_path);
    assert_eq!(
        stdout,
        "class: Document\nchildren:\n- class: Paragraph\n  children:\n  - class: PlainText\n    attributes:\n      text: |\n        Hello"
    );
}

#[test]
fn convert_rejects_unsupported_output_formats() {
    let output = Command::new(env!("CARGO_BIN_EXE_rstu"))
        .args(["convert", "missing.rst", "-o", "toml"])
        .output()
        .expect("failed to run rstu binary");

    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid value"));
}
