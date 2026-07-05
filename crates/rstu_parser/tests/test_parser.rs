// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use document_tree::HasChildren;
use rst_parser::parse;
use std::fs;
use std::path::Path;

#[test]
fn parses_lorem_ipsum_document_tree() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read lorem ipsum test file");

    let document = parse(&contents).expect("failed to parse lorem ipsum fixture");

    assert_eq!(document.children().len(), 1);
    let rendered = format!("{document:#?}");
    println!("{rendered}");
    assert!(rendered.contains("Lorem Ipsum Heading"));
    assert!(rendered.contains("Lorem ipsum"));
}
