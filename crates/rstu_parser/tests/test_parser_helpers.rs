// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[path = "common/mod.rs"]
mod test_parser;

#[test]
fn test_truncate_str() {
    assert_eq!(test_parser::truncate_str("short", 10), "short");
    assert_eq!(test_parser::truncate_str("exactly ten", 11), "exactly ten");
    assert_eq!(
        test_parser::truncate_str("this is longer than 10", 10),
        "this is..."
    );
}

#[test]
fn test_first_diff_line() {
    let a = "line1\nline2\nline3";
    let b = "line1\nline2_diff\nline3";
    assert_eq!(test_parser::first_diff_line(a, b), Some(2));

    let a2 = "line1\nline2";
    let b2 = "line1\nline2\nline3";
    assert_eq!(test_parser::first_diff_line(a2, b2), Some(3));
}

#[test]
fn test_format_side_by_side_window() {
    let actual = (1..=20)
        .map(|index| {
            if index == 10 {
                format!(
                    "line {} act with very long text exceeding max width limit",
                    index
                )
            } else {
                format!("line {}", index)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let expected = (1..=20)
        .map(|index| {
            if index == 10 {
                format!(
                    "line {} exp with very long text exceeding max width limit",
                    index
                )
            } else {
                format!("line {}", index)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    let result = test_parser::format_side_by_side(&actual, &expected, 10, 30);
    assert!(result.contains("   7 |"));
    assert!(result.contains("  15 |"));
    assert!(!result.contains("   6 |"));
    assert!(!result.contains("  16 |"));
    assert!(result.contains(">   10 |"));
    assert!(result.contains("line 10 act with very long ..."));
    assert!(result.contains("line 10 exp with very long ..."));
}
