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

                    entries.sort_by(|(ka, _), (kb, _)| yaml_sort_key(ka).cmp(&yaml_sort_key(kb)));

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

        assert_eq!(
            actual_value, expected_value,
            "Unexpected parse output for {}",
            $rst_filename
        );
    }};
}
