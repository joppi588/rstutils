// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use document_tree::{
    HasChildren,
    attribute_types::Measure,
    element_categories::{BodyElement, StructuralSubElement, SubStructure, TextOrInlineElement},
    extra_attributes::ExtraAttributes,
};
use rst_parser::parse;
use std::fs;
use std::path::{Path, PathBuf};

fn test_data_path(filename: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(filename)
}

#[test]
fn test_parses_lorem_ipsum_document_tree() {
    // GIVEN An error-free RST file
    // WHEN The file is parsed
    // THEN The document tree has heading, tile and bold text
    let contents = fs::read_to_string(test_data_path("ok_mixed_lorem_ipsum.rst"))
        .expect("failed to read lorem ipsum test file");
    let document = parse(&contents).expect("failed to parse lorem ipsum example");

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
        .find_map(|c| {
            if let StructuralSubElement::Title(t) = c {
                Some(t)
            } else {
                None
            }
        })
        .expect("expected a title element");
    let title_text = title.children().iter().find_map(|c| {
        if let TextOrInlineElement::String(s) = c {
            Some(s.as_str())
        } else {
            None
        }
    });
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
        .find_map(|c| {
            if let TextOrInlineElement::Strong(s) = c {
                Some(s)
            } else {
                None
            }
        })
        .and_then(|strong| {
            strong.children().iter().find_map(|c| {
                if let TextOrInlineElement::String(s) = c {
                    Some(s.as_str())
                } else {
                    None
                }
            })
        });
    assert_eq!(strong_text, Some("end of file"));
}

#[test]
fn test_parses_numeric_image_options() {
    // GIVEN An RST file with numeric image options
    // WHEN The file is parsed
    // THEN The image node exposes height, width and scale
    let contents = fs::read_to_string(test_data_path("ok_image_numeric_options.rst"))
        .expect("failed to read image options test file");
    let document = parse(&contents).expect("failed to parse image options example");

    let image = document
        .children()
        .iter()
        .find_map(|child| {
            let StructuralSubElement::SubStructure(sub) = child else {
                return None;
            };
            let SubStructure::BodyElement(body) = sub.as_ref() else {
                return None;
            };
            let BodyElement::Image(image) = body.as_ref() else {
                return None;
            };
            Some(image)
        })
        .expect("expected an image node");

    assert_eq!(image.extra().height, Some(Measure::Mm(20.0)));
    assert_eq!(image.extra().width, Some(Measure::Px(300.0)));
    assert_eq!(image.extra().scale, Some(75));
}
