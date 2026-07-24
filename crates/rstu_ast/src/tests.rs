// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::relink_parent_pointers;
use super::{AstNode, ElementKind, Node};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct FixtureNode {
    kind: ElementKind,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    children: Vec<FixtureNode>,
}

fn node_from_fixture(src: FixtureNode) -> Node {
    let children = src.children.into_iter().map(node_from_fixture).collect();
    Node {
        kind: src.kind,
        parent: None,
        attributes: src.attributes,
        text: src.text,
        children,
    }
}

fn load_document_fixture(file_name: &str) -> Box<Node> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(file_name);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path.display(), e));
    let fixture: FixtureNode =
        serde_yaml::from_str(&raw).unwrap_or_else(|e| panic!("Invalid fixture YAML: {}", e));
    let mut root = Box::new(node_from_fixture(fixture));
    relink_parent_pointers(root.as_mut());
    root
}

fn section_with_marker(section_marker: &str) -> Node {
    Node::new(ElementKind::Section).with_attr("section_marker", section_marker)
}

#[test]
fn validates_minimal_document_with_section_and_paragraph() {
    let tree = load_document_fixture("valid_document_section_paragraph.yaml");

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_invalid_list_child() {
    let tree = load_document_fixture("invalid_list_child.yaml");

    assert!(tree.validate().is_err());
}

#[test]
fn validates_figure_with_image_and_caption() {
    let tree = load_document_fixture("valid_figure_with_image_caption.yaml");

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_text_inside_empty_element() {
    let tree = load_document_fixture("invalid_meta_text.yaml");

    assert!(tree.validate().is_err());
}

#[test]
fn rejects_non_inline_child_in_paragraph() {
    let tree = load_document_fixture("invalid_non_inline_child.yaml");

    assert!(tree.validate().is_err());
}

#[test]
fn push_section_pushes_into_root_when_called_on_root() {
    let mut tree = load_document_fixture("document_root.yaml");
    let section = section_with_marker("#");

    tree.push_section(section).unwrap();

    assert_eq!(tree.children.len(), 1);
    assert_eq!(tree.children[0].kind, ElementKind::Section);
}

#[test]
fn push_section_with_same_marker_pushes_to_parent_of_self() {
    let mut tree = load_document_fixture("section_single_hash.yaml");

    {
        let current_mut = &mut tree.children[0];
        let section = section_with_marker("#");
        current_mut.push_section(section).unwrap();
    }

    assert_eq!(tree.children.len(), 2);
    assert_eq!(
        tree.children[1]
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_with_ancestor_marker_pushes_to_parent_of_matching_ancestor() {
    let mut tree = load_document_fixture("section_nested_hash_tilde.yaml");

    {
        let inner_mut = &mut tree.children[0].children[0];
        let section = section_with_marker("#");
        inner_mut.push_section(section).unwrap();
    }

    assert_eq!(tree.children.len(), 2);
    assert_eq!(
        tree.children[1]
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_without_marker_match_pushes_to_closest_ancestor_section() {
    let mut tree = load_document_fixture("section_nested_hash_tilde.yaml");

    {
        let paragraph_mut = &mut tree.children[0].children[0].children[0];
        let section = section_with_marker("^");
        paragraph_mut.push_section(section).unwrap();
    }

    let inner_section = &tree.children[0].children[0];
    assert_eq!(inner_section.children.len(), 2);
    assert_eq!(inner_section.children[1].kind, ElementKind::Section);
    assert_eq!(
        inner_section.children[1]
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("^")
    );
}

#[test]
fn closest_ancestor_section_finds_nearest_section_upwards() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let paragraph = &tree.children[0].children[0].children[0];
    let closest = paragraph.closest_ancestor_section(None).unwrap();

    assert_eq!(closest.kind, ElementKind::Section);
    assert_eq!(
        closest.attributes.get("section_marker").map(String::as_str),
        Some("~")
    );
}

#[test]
fn closest_ancestor_section_returns_none_at_root() {
    let tree = load_document_fixture("document_root.yaml");
    assert!(tree.closest_ancestor_section(None).is_none());
}

#[test]
fn closest_ancestor_section_matches_requested_marker() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let paragraph = &tree.children[0].children[0].children[0];
    let closest = paragraph.closest_ancestor_section(Some("#")).unwrap();

    assert_eq!(closest.kind, ElementKind::Section);
    assert_eq!(
        closest.attributes.get("section_marker").map(String::as_str),
        Some("#")
    );
}

#[test]
fn to_json_serializes_node_tree_without_parent() {
    let mut root = Node::new(ElementKind::Document).with_attr("lang", "rst");
    let mut section = Node::new(ElementKind::Section)
        .with_attr("opening_style", "=========")
        .with_attr("closing_style", "=========");
    section.with_child(Node::new(ElementKind::Title).with_text("Heading 1\n"));
    root.with_child(section);

    let json_value = root.to_json();

    assert_eq!(
        json_value,
        json!({
            "kind": "Document",
            "attributes": {
                "lang": "rst"
            },
            "text": null,
            "children": [
                {
                    "kind": "Section",
                    "attributes": {
                        "closing_style": "=========",
                        "opening_style": "========="
                    },
                    "text": null,
                    "children": [
                        {
                            "kind": "Title",
                            "attributes": {},
                            "text": "Heading 1\n",
                            "children": []
                        }
                    ]
                }
            ]
        })
    );
}

#[test]
fn to_yaml_serializes_node_tree_without_parent() {
    let mut root = Node::new(ElementKind::Document).with_attr("lang", "rst");
    let mut section = Node::new(ElementKind::Section)
        .with_attr("opening_style", "=========")
        .with_attr("closing_style", "=========");
    section.with_child(Node::new(ElementKind::Title).with_text("Heading 1\n"));
    root.with_child(section);

    let yaml_text = root.to_yaml().expect("failed to serialize yaml");
    let actual: serde_yaml::Value =
        serde_yaml::from_str(&yaml_text).expect("failed to parse generated yaml");
    let expected: serde_yaml::Value = serde_yaml::from_str(
        r#"kind: Document
attributes:
  lang: rst
text: null
children:
  - kind: Section
    attributes:
      closing_style: =========
      opening_style: =========
    text: null
    children:
      - kind: Title
        attributes: {}
        text: "Heading 1\n"
        children: []
"#,
    )
    .expect("failed to parse expected yaml");

    assert_eq!(actual, expected);
}

#[test]
fn ast_node_ref_roundtrip_matches_owned_node_json() {
    let mut root = Node::new(ElementKind::Document).with_attr("lang", "rst");
    let mut section = Node::new(ElementKind::Section)
        .with_attr("opening_style", "=========")
        .with_attr("closing_style", "=========");
    section.with_child(Node::new(ElementKind::Title).with_text("Heading 1\n"));
    root.with_child(section);

    let root_ref = AstNode::from_node(root.clone());
    let roundtrip = AstNode::to_node(&root_ref);

    assert_eq!(roundtrip.to_json(), root.to_json());
}

#[test]
fn push_section_ref_returns_inserted_current_node() {
    let document = AstNode::new_ref(ElementKind::Document);

    let section_one = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_one, "section_marker", "#");
    let title_one = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title_one, "Heading 1\n");
    AstNode::push_child(&section_one, title_one).unwrap();

    let mut current = AstNode::push_section_ref(&document, section_one).unwrap();

    let section_two = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_two, "section_marker", "~");
    let title_two = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title_two, "Heading 2\n");
    AstNode::push_child(&section_two, title_two).unwrap();

    current = AstNode::push_section_ref(&current, section_two).unwrap();

    let borrowed = current.borrow();
    assert_eq!(borrowed.kind, ElementKind::Section);
    assert_eq!(
        borrowed
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("~")
    );
}
