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
