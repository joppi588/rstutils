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

        fn yaml_remainder(
            actual: &serde_yaml::Value,
            expected: &serde_yaml::Value,
        ) -> Option<(serde_yaml::Value, serde_yaml::Value)> {
            match (actual, expected) {
                (serde_yaml::Value::Mapping(a), serde_yaml::Value::Mapping(e)) => {
                    let mut only_actual = serde_yaml::Mapping::new();
                    let mut only_expected = serde_yaml::Mapping::new();

                    for (k, av) in a {
                        match e.get(k) {
                            Some(ev) => {
                                if let Some((da, de)) = yaml_remainder(av, ev) {
                                    only_actual.insert(k.clone(), da);
                                    only_expected.insert(k.clone(), de);
                                }
                            }
                            None => {
                                only_actual.insert(k.clone(), av.clone());
                            }
                        }
                    }

                    for (k, ev) in e {
                        if !a.contains_key(k) {
                            only_expected.insert(k.clone(), ev.clone());
                        }
                    }

                    if only_actual.is_empty() && only_expected.is_empty() {
                        None
                    } else {
                        Some((
                            serde_yaml::Value::Mapping(only_actual),
                            serde_yaml::Value::Mapping(only_expected),
                        ))
                    }
                }
                (serde_yaml::Value::Sequence(a), serde_yaml::Value::Sequence(e)) => {
                    let mut only_actual = Vec::new();
                    let mut only_expected = Vec::new();
                    let min_len = a.len().min(e.len());

                    for idx in 0..min_len {
                        if let Some((da, de)) = yaml_remainder(&a[idx], &e[idx]) {
                            only_actual.push(da);
                            only_expected.push(de);
                        }
                    }

                    if a.len() > min_len {
                        only_actual.extend(a[min_len..].iter().cloned());
                    }
                    if e.len() > min_len {
                        only_expected.extend(e[min_len..].iter().cloned());
                    }

                    if only_actual.is_empty() && only_expected.is_empty() {
                        None
                    } else {
                        Some((
                            serde_yaml::Value::Sequence(only_actual),
                            serde_yaml::Value::Sequence(only_expected),
                        ))
                    }
                }
                _ if actual == expected => None,
                _ => Some((actual.clone(), expected.clone())),
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

        if actual_value != expected_value {
            let (actual_remainder, expected_remainder) =
                yaml_remainder(&actual_value, &expected_value)
                    .unwrap_or_else(|| (actual_value.clone(), expected_value.clone()));
            let actual_pretty = serde_yaml::to_string(&actual_remainder)
                .expect("failed to pretty-print actual yaml remainder");
            let expected_pretty = serde_yaml::to_string(&expected_remainder)
                .expect("failed to pretty-print expected yaml remainder");

            panic!(
                "Unexpected parse output for {}\n\nActual\n[...]\n{}\nExpected\n[...]\n{}",
                $rst_filename, actual_pretty, expected_pretty
            );
        }

        assert_eq!(
            actual_value, expected_value,
            "Unexpected parse output for {}",
            $rst_filename
        );
    }};
}
