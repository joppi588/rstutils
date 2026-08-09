// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[macro_export]
macro_rules! rst_vs_yaml {
    ($directory:expr, $rst_filename: expr,$yaml_filename:expr) => {{
        fn yaml_sort_key(key: &serde_yaml::Value) -> String {
            match key {
                serde_yaml::Value::String(s) => format!("s:{s}"),
                _ => serde_yaml::to_string(key).unwrap_or_else(|_| format!("{key:?}")),
            }
        }

        fn canonicalize_yaml(value: &mut serde_yaml::Value) {
            match value {
                serde_yaml::Value::Mapping(mapping) => {
                    let mut entries: Vec<(serde_yaml::Value, serde_yaml::Value)> = mapping
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect();

                    for (_, v) in &mut entries {
                        canonicalize_yaml(v);
                    }

                    entries.sort_by(|(ka, _), (kb, _)| {
                        let a_is_children = matches!(ka, &serde_yaml::Value::String(ref key) if key == "children");
                        let b_is_children = matches!(kb, &serde_yaml::Value::String(ref key) if key == "children");

                        match (a_is_children, b_is_children) {
                            (true, false) => std::cmp::Ordering::Greater,
                            (false, true) => std::cmp::Ordering::Less,
                            _ => yaml_sort_key(ka).cmp(&yaml_sort_key(kb)),
                        }
                    });

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

        fn first_diff_line(actual: &str, expected: &str) -> Option<usize> {
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

        fn line_at<'a>(lines: &'a [&'a str], line_number: usize) -> &'a str {
            if line_number == 0 {
                return "";
            }

            lines.get(line_number - 1).copied().unwrap_or("<missing>")
        }

        fn format_context(yaml: &str, start_line: usize, follow_lines: usize) -> String {
            let lines: Vec<&str> = yaml.lines().collect();
            if start_line == 0 || lines.is_empty() {
                return "<no context available>".to_string();
            }

            let start_index = start_line.saturating_sub(1);
            if start_index >= lines.len() {
                return format!("{:>4}: <missing>", start_line);
            }

            let end_exclusive = (start_index + follow_lines + 1).min(lines.len());
            let mut out = Vec::new();
            for index in start_index..end_exclusive {
                out.push(format!("{:>4}: {}", index + 1, lines[index]));
            }
            out.join("\n")
        }

        let rst_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data")
            .join($directory)
            .join($rst_filename);
        let rst_contents = fs::read_to_string(&rst_path)
            .unwrap_or_else(|_| panic!("failed to read sections test file: {}", $rst_filename));

        let parsed = parse(&rst_contents).expect("expected parse to succeed");
        let actual_yaml =
            AstNode::to_yaml(&parsed).expect("failed to serialize parse output to yaml");

        let expected_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/data")
            .join($directory)
            .join($yaml_filename);
        let expected_yaml = fs::read_to_string(&expected_path)
            .unwrap_or_else(|_| panic!("failed to read expected yaml fixture: {}", $yaml_filename));

        let mut actual_value: serde_yaml::Value =
            serde_yaml::from_str(&actual_yaml).expect("failed to parse generated yaml");
        let mut expected_value: serde_yaml::Value =
            serde_yaml::from_str(&expected_yaml).expect("failed to parse expected yaml fixture");

        canonicalize_yaml(&mut actual_value);
        canonicalize_yaml(&mut expected_value);

        if actual_value != expected_value {
            let actual_canonical = serde_yaml::to_string(&actual_value)
                .expect("failed to serialize canonical actual yaml");
            let expected_canonical = serde_yaml::to_string(&expected_value)
                .expect("failed to serialize canonical expected yaml");

            let diff_line = first_diff_line(&actual_canonical, &expected_canonical).unwrap_or(1);
            let actual_lines: Vec<&str> = actual_canonical.lines().collect();
            let expected_lines: Vec<&str> = expected_canonical.lines().collect();
            let actual_line = line_at(&actual_lines, diff_line);
            let expected_line = line_at(&expected_lines, diff_line);
            let actual_context = format_context(&actual_canonical, diff_line, 5);
            let expected_context = format_context(&expected_canonical, diff_line, 5);

            panic!(
                "Unexpected parse output for {}\n\nFirst deviation at canonicalized line {}\nActual line: {}\nExpected line: {}\n\nActual context\n{}\n\nExpected context\n{}",
                $rst_filename,
                diff_line,
                actual_line,
                expected_line,
                actual_context,
                expected_context
            );
        }

        assert_eq!(
            actual_value, expected_value,
            "Unexpected parse output for {}",
            $rst_filename
        );
    }};
}
