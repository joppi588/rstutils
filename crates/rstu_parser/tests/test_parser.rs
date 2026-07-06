// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use document_tree::{
    HasChildren,
    element_categories::{BodyElement, StructuralSubElement, SubStructure, TextOrInlineElement},
};
use rst_parser::parse;
use std::fs;
use std::path::{Path, PathBuf};

fn test_data_path(filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data").join(filename)
}

#[test]
fn test_parses_lorem_ipsum_document_tree() {
    // GIVEN An error-free RST file 
    // WHEN The file is parsed
    // THEN The document tree has heading, tile and bold text
    let contents = fs::read_to_string(test_data_path("ok_mixed_lorem_ipsum.rst"))
        .expect("failed to read lorem ipsum test file");
    let document = parse(&contents).expect("failed to parse lorem ipsum fixture");

    assert_eq!(document.children().len(), 1);

    let StructuralSubElement::SubStructure(sub) = document.children().first().unwrap() else {
        panic!("expected a section node");
    };
    let SubStructure::Section(section) = sub.as_ref() else {
        panic!("expected a section node");
    };

    let children = section.children();
    let title = children
        .iter()
        .find_map(|c| if let StructuralSubElement::Title(t) = c { Some(t) } else { None })
        .expect("expected a title element");
    let title_text = title
        .children()
        .iter()
        .find_map(|c| if let TextOrInlineElement::String(s) = c { Some(s.as_str()) } else { None });
    assert_eq!(title_text, Some("Lorem Ipsum Heading"));

    let StructuralSubElement::SubStructure(sub) = children.last().unwrap() else {
        panic!("expected a paragraph as the last child");
    };
    let SubStructure::BodyElement(body) = sub.as_ref() else {
        panic!("expected a paragraph as the last child");
    };
    let BodyElement::Paragraph(paragraph) = body.as_ref() else {
        panic!("expected a paragraph as the last child");
    };

    let strong_text = paragraph
        .children()
        .iter()
        .find_map(|c| if let TextOrInlineElement::Strong(s) = c { Some(s) } else { None })
        .and_then(|strong| {
            strong
                .children()
                .iter()
                .find_map(|c| if let TextOrInlineElement::String(s) = c { Some(s.as_str()) } else { None })
        });
    assert_eq!(strong_text, Some("end of file"));
}
