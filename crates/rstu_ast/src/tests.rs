// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{AstNode, NodeClass, NodeRefExt};
use serde_json::json;
use std::rc::Rc;

fn section_with_marker(section_marker: &str) -> super::NodeRef {
    let section = AstNode::new_ref(NodeClass::Section);
    section.with_attr("section_marker", section_marker);
    section
}

#[test]
fn push_child_sets_parent_and_appends_child() {
    let parent = AstNode::new_ref(NodeClass::Document);
    let child = AstNode::new_ref(NodeClass::Paragraph);

    parent.push_child(child.clone());

    let borrowed = parent.borrow();
    assert_eq!(borrowed.children.len(), 1);
    assert!(Rc::ptr_eq(&borrowed.children[0], &child));
    assert!(child.borrow().parent.is_some());
}

#[test]
fn push_section_with_same_marker_pushes_to_parent_of_self() {
    let tree = AstNode::new_ref(NodeClass::Document);
    let current = section_with_marker("#");
    tree.push_child(current.clone());

    let section = section_with_marker("#");
    current.push_section_ref(section);

    let borrowed = tree.borrow();
    assert_eq!(borrowed.children.len(), 2);
    assert_eq!(borrowed.children[1].borrow().class, NodeClass::Section);
}

#[test]
fn closest_ancestor_section_matches_requested_marker() {
    let document = AstNode::new_ref(NodeClass::Document);
    let outer = section_with_marker("#");
    document.push_child(outer.clone());
    let inner = section_with_marker("~");
    outer.push_child(inner.clone());
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    inner.push_child(paragraph.clone());

    let closest = AstNode::closest_ancestor_section(&paragraph, Some("#")).unwrap();
    assert!(Rc::ptr_eq(&closest, &outer));
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
