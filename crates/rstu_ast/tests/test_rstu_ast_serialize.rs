// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::AstNode;
use rstu_ast::{NodeClass, NodeRefExt};
use serde_json::json;
use std::rc::Rc;

fn section_with_marker(section_marker: &str) -> rstu_ast::NodeRef {
    let section = AstNode::new_ref(NodeClass::Section);
    section.with_attr("section_marker", section_marker);
    section
}

#[test]
fn to_json_serializes_node_tree_without_parent() {
    let root = AstNode::new_ref(NodeClass::Document);
    root.with_attr("lang", "rst");

    let section = AstNode::new_ref(NodeClass::Section);
    section
        .with_attr("section_marker", "=========")
        .with_attr("marker_len", 9usize);

    let title = AstNode::new_ref(NodeClass::Title);
    title.with_attr("text", "Heading 1\n");
    section.push_child(title);
    root.push_child(section);

    let json_value = AstNode::to_json(&root);

    assert_eq!(
        json_value,
        json!({
            "class": "Document",
            "attributes": {
                "lang": "rst"
            },
            "children": [
                {
                    "class": "Section",
                    "attributes": {
                        "section_marker": "=========",
                        "marker_len": 9
                    },
                    "children": [
                        {
                            "class": "Title",
                            "attributes": {
                                "text": "Heading 1\n",
                            },
                        }
                    ]
                }
            ]
        })
    );
}

#[test]
fn to_yaml_serializes_node_tree_without_parent() {
    let root = AstNode::new_ref(NodeClass::Document);
    root.with_attr("lang", "rst");

    let section = AstNode::new_ref(NodeClass::Section);
    section
        .with_attr("section_marker", "=========")
        .with_attr("marker_len", 9usize);

    let title = AstNode::new_ref(NodeClass::Title);
    title.with_attr("text", "Heading 1\n");
    section.push_child(title);
    root.push_child(section);

    let yaml_text = AstNode::to_yaml(&root).expect("failed to serialize yaml");
    let actual: serde_yaml::Value =
        serde_yaml::from_str(&yaml_text).expect("failed to parse generated yaml");
    let expected = serde_yaml::to_value(json!({
        "class": "Document",
        "attributes": { "lang": "rst" },
        "children": [{
            "class": "Section",
            "attributes": {
                "section_marker": "=========",
                "marker_len": 9,
            },
            "children": [{
                "class": "Title",
                "attributes": { "text": "Heading 1\n" },
            }],
        }],
    }))
    .expect("failed to construct expected yaml");

    assert_eq!(actual, expected);
}

#[test]
fn serializers_preserve_node_and_attribute_key_order() {
    let root = AstNode::new_ref(NodeClass::Document);
    root.with_attr("zeta", "last").with_attr("alpha", "first");

    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    paragraph
        .with_attr("zeta", "last")
        .with_attr("alpha", "first");
    paragraph.push_child(AstNode::new_ref(NodeClass::Title));
    let section = AstNode::new_ref(NodeClass::Section);
    root.push_child(paragraph);
    root.push_child(section);

    let json = serde_json::to_string(&AstNode::to_json(&root)).unwrap();
    assert_eq!(
        json,
        r#"{"class":"Document","attributes":{"alpha":"first","zeta":"last"},"children":[{"class":"Paragraph","attributes":{"alpha":"first","zeta":"last"},"children":[{"class":"Title"}]},{"class":"Section"}]}"#
    );

    let yaml = AstNode::to_yaml(&root).unwrap();
    assert_eq!(
        yaml,
        "class: Document\nattributes:\n  alpha: first\n  zeta: last\nchildren:\n- class: Paragraph\n  attributes:\n    alpha: first\n    zeta: last\n  children:\n  - class: Title\n- class: Section\n"
    );
}
