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
fn parses_lorem_ipsum_document_tree() {
    let path = test_data_path("ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read lorem ipsum test file");

    let document = parse(&contents).expect("failed to parse lorem ipsum fixture");

    assert_eq!(document.children().len(), 1);

    let section = document.children().first().expect("expected a section");
    let section = match section {
        StructuralSubElement::SubStructure(substructure) => match substructure.as_ref() {
            SubStructure::Section(section) => section,
            _ => panic!("expected a section node"),
        },
        _ => panic!("expected a section node"),
    };

    let section_children = section.children();
    let title = section_children
        .iter()
        .find_map(|child| match child {
            StructuralSubElement::Title(title) => Some(title),
            _ => None,
        })
        .expect("expected a title element");
    let title_text = title.children().iter().find_map(|child| match child {
        TextOrInlineElement::String(text) => Some(text.as_str()),
        _ => None,
    });
    assert_eq!(title_text, Some("Lorem Ipsum Heading"));

    let last_child = section_children.last().expect("expected a trailing child");
    let paragraph = match last_child {
        StructuralSubElement::SubStructure(substructure) => match substructure.as_ref() {
            SubStructure::BodyElement(body_element) => match body_element.as_ref() {
                BodyElement::Paragraph(paragraph) => Some(paragraph),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
    .expect("expected a paragraph as the last child");

    let strong_text = paragraph.children().iter().find_map(|child| match child {
        TextOrInlineElement::Strong(strong) => Some(strong.children().iter().find_map(|grandchild| match grandchild {
            TextOrInlineElement::String(text) => Some(text.as_str()),
            _ => None,
        })),
        _ => None,
    })
    .flatten();
    assert_eq!(strong_text, Some("end of file"));
}
