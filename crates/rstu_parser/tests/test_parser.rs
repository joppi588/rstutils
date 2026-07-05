// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::{Document, NodeKind, parse};
use std::fs;
use std::path::Path;

#[test]
fn parses_lorem_ipsum_document_tree() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read lorem ipsum test file");

    let document = parse(&contents);

    assert_eq!(document.children.len(), 2);
    assert_eq!(document.children[0].kind, NodeKind::Heading { level: 1 });
    assert_eq!(document.children[0].text.as_deref(), Some("Lorem Ipsum Heading"));
    assert_eq!(document.children[1].kind, NodeKind::Note);

    let note = &document.children[1];
    assert_eq!(note.children.len(), 1);
    assert_eq!(note.children[0].kind, NodeKind::Paragraph);
    assert!(note.children[0]
        .text
        .as_ref()
        .unwrap()
        .contains("Lorem ipsum"));
}
