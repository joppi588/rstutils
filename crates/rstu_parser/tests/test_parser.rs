// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[allow(dead_code)]
pub fn yaml_field_order(key: &serde_yaml::Value) -> usize {
    match key {
        serde_yaml::Value::String(s) if s == "class" => 0,
        serde_yaml::Value::String(s) if s == "attributes" => 1,
        serde_yaml::Value::String(s) if s == "children" => 2,
        _ => 3,
    }
}

#[allow(dead_code)]
pub fn canonicalize_yaml(value: &mut serde_yaml::Value) {
    match value {
        serde_yaml::Value::Mapping(mapping) => {
            let mut entries: Vec<(serde_yaml::Value, serde_yaml::Value)> = mapping
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            for (_, v) in &mut entries {
                canonicalize_yaml(v);
            }

            entries.sort_by(|(ka, _), (kb, _)| yaml_field_order(ka).cmp(&yaml_field_order(kb)));

            mapping.clear();
            for (k, v) in entries {
                mapping.insert(k, v);
            }
        }
        serde_yaml::Value::Sequence(seq) => {
            for item in seq {
                canonicalize_yaml(item);
            }
        }
        _ => {}
    }
}

#[allow(dead_code)]
pub fn first_diff_line(actual: &str, expected: &str) -> Option<usize> {
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let min_len = actual_lines.len().min(expected_lines.len());

    for index in 0..min_len {
        if actual_lines[index] != expected_lines[index] {
            return Some(index + 1);
        }
    }

    if actual_lines.len() != expected_lines.len() {
        Some(min_len + 1)
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn line_at<'a>(lines: &'a [&'a str], line_number: usize) -> &'a str {
    if line_number == 0 {
        return "";
    }

    lines.get(line_number - 1).copied().unwrap_or("<missing>")
}

#[allow(dead_code)]
pub fn truncate_str(s: &str, max_width: usize) -> String {
    let char_count = s.chars().count();
    if char_count <= max_width {
        s.to_string()
    } else {
        let keep = max_width.saturating_sub(3);
        let prefix: String = s.chars().take(keep).collect();
        format!("{}...", prefix)
    }
}

#[allow(dead_code)]
pub fn format_side_by_side(
    actual: &str,
    expected: &str,
    diff_line: usize,
    max_width: usize,
) -> String {
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const BOLD_RED: &str = "\x1b[1;31m";
    const BOLD_GREEN: &str = "\x1b[1;32m";
    const DIM: &str = "\x1b[2m";

    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    let total_lines = actual_lines.len().max(expected_lines.len());

    if total_lines == 0 {
        return "<no context available>".to_string();
    }

    let start_line = diff_line.saturating_sub(3).max(1);
    let end_line = (diff_line + 5).min(total_lines);

    let left_header = format!("  {:>4} | {:<width$}", "Line", "Actual", width = max_width);
    let right_header = format!(
        "  {:>4} | {:<width$}",
        "Line",
        "Expected",
        width = max_width
    );
    let header = format!(
        "{}{}{} | {}{}{}",
        BOLD, left_header, RESET, BOLD, right_header, RESET
    );

    let left_sep = format!("-------+-{}", "-".repeat(max_width));
    let right_sep = format!("-------+-{}", "-".repeat(max_width));
    let separator = format!("{}{}-+-{}{}", DIM, left_sep, right_sep, RESET);

    let mut rows = Vec::new();
    rows.push(header);
    rows.push(separator);

    for line_num in start_line..=end_line {
        let idx = line_num - 1;
        let act_raw = actual_lines.get(idx).copied();
        let exp_raw = expected_lines.get(idx).copied();

        let is_diff = act_raw != exp_raw;

        let act_text = act_raw.unwrap_or("<missing>");
        let exp_text = exp_raw.unwrap_or("<missing>");

        let act_truncated = truncate_str(act_text, max_width);
        let exp_truncated = truncate_str(exp_text, max_width);

        let act_padded = format!("{:<width$}", act_truncated, width = max_width);
        let exp_padded = format!("{:<width$}", exp_truncated, width = max_width);

        let (act_col, exp_col) = if is_diff {
            let act_col = format!(
                "{}> {:>4} | {}{}{}",
                BOLD_RED, line_num, RED, act_padded, RESET
            );
            let exp_col = format!(
                "{}> {:>4} | {}{}{}",
                BOLD_GREEN, line_num, GREEN, exp_padded, RESET
            );
            (act_col, exp_col)
        } else {
            let act_col = format!("{}  {:>4} | {}{}", DIM, line_num, act_padded, RESET);
            let exp_col = format!("{}  {:>4} | {}{}", DIM, line_num, exp_padded, RESET);
            (act_col, exp_col)
        };

        rows.push(format!("{} | {}", act_col, exp_col));
    }

    rows.join("\n")
}

#[macro_export]
macro_rules! rst_vs_yaml {
    ($directory:expr, $test_case:expr) => {{
        let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data")
            .join($directory)
            .join(format!("{}.rst", $test_case));
        let rst_contents = fs::read_to_string(&rst_path)
            .unwrap_or_else(|_| panic!("failed to read sections test file: {}", $test_case));

        let parsed = parse(&rst_contents).expect("expected parse to succeed");
        let actual_yaml =
            AstNode::to_yaml(&parsed).expect("failed to serialize parse output to yaml");

        let expected_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data")
            .join($directory)
            .join(format!("{}.yaml", $test_case));
        let expected_yaml = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("failed to read expected yaml fixture: {}", $test_case));

        let mut actual_value: serde_yaml::Value =
            serde_yaml::from_str(&actual_yaml).expect("failed to parse generated yaml");
        let mut expected_value: serde_yaml::Value =
            serde_yaml::from_str(&expected_yaml).expect("failed to parse expected yaml fixture");

        test_parser::canonicalize_yaml(&mut actual_value);
        test_parser::canonicalize_yaml(&mut expected_value);

        if actual_value != expected_value {
            let actual_canonical = serde_yaml::to_string(&actual_value)
                .expect("failed to serialize canonical actual yaml");
            let expected_canonical = serde_yaml::to_string(&expected_value)
                .expect("failed to serialize canonical expected yaml");

            let diff_line = test_parser::first_diff_line(&actual_canonical, &expected_canonical).unwrap_or(1);
            let actual_lines: Vec<&str> = actual_canonical.lines().collect();
            let expected_lines: Vec<&str> = expected_canonical.lines().collect();
            let actual_line = test_parser::line_at(&actual_lines, diff_line);
            let expected_line = test_parser::line_at(&expected_lines, diff_line);
            let side_by_side = test_parser::format_side_by_side(&actual_canonical, &expected_canonical, diff_line, 45);

            panic!(
                "\n\nUnexpected parse output for fixture: \x1b[1;33m{}\x1b[0m\n\nFirst deviation at canonicalized line \x1b[1;31m{}\x1b[0m\nActual line:   \x1b[31m{}\x1b[0m\nExpected line: \x1b[32m{}\x1b[0m\n\n{}\n",
                $test_case,
                diff_line,
                test_parser::truncate_str(actual_line, 60),
                test_parser::truncate_str(expected_line, 60),
                side_by_side
            );
        }

        assert_eq!(
            actual_value, expected_value,
            "Unexpected parse output for {}",
            $test_case
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate_str() {
        assert_eq!(truncate_str("short", 10), "short");
        assert_eq!(truncate_str("exactly ten", 11), "exactly ten");
        assert_eq!(truncate_str("this is longer than 10", 10), "this is...");
    }

    #[test]
    fn test_first_diff_line() {
        let a = "line1\nline2\nline3";
        let b = "line1\nline2_diff\nline3";
        assert_eq!(first_diff_line(a, b), Some(2));

        let a2 = "line1\nline2";
        let b2 = "line1\nline2\nline3";
        assert_eq!(first_diff_line(a2, b2), Some(3));
    }

    #[test]
    fn test_format_side_by_side_window() {
        let actual = (1..=20)
            .map(|i| {
                if i == 10 {
                    format!(
                        "line {} act with very long text exceeding max width limit",
                        i
                    )
                } else {
                    format!("line {}", i)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let expected = (1..=20)
            .map(|i| {
                if i == 10 {
                    format!(
                        "line {} exp with very long text exceeding max width limit",
                        i
                    )
                } else {
                    format!("line {}", i)
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        let result = format_side_by_side(&actual, &expected, 10, 30);
        // Start line: 10 - 3 = 7, end line: 10 + 5 = 15
        assert!(result.contains("   7 |"));
        assert!(result.contains("  15 |"));
        assert!(!result.contains("   6 |"));
        assert!(!result.contains("  16 |"));

        // Check difference marker on line 10
        assert!(result.contains(">   10 |"));
        // Check truncation of long lines
        assert!(result.contains("line 10 act with very long ..."));
        assert!(result.contains("line 10 exp with very long ..."));
    }
}
