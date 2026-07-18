// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::ptr::NonNull;

use super::{ElementKind, Node};

#[derive(Debug, Deserialize)]
struct JsonNode {
    kind: String,
    #[serde(default)]
    attributes: BTreeMap<String, String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    children: Vec<JsonNode>,
}

fn kind_from_str(kind: &str) -> ElementKind {
    match kind {
        "Document" => ElementKind::Document,
        "Section" => ElementKind::Section,
        "Title" => ElementKind::Title,
        "Paragraph" => ElementKind::Paragraph,
        "BulletList" => ElementKind::BulletList,
        "ListItem" => ElementKind::ListItem,
        "Figure" => ElementKind::Figure,
        "Image" => ElementKind::Image,
        "Caption" => ElementKind::Caption,
        "Meta" => ElementKind::Meta,
        other => panic!("Unsupported ElementKind in fixture: {}", other),
    }
}

fn node_from_fixture(src: JsonNode) -> Node {
    let children = src.children.into_iter().map(node_from_fixture).collect();
    Node {
        kind: kind_from_str(&src.kind),
        parent: None,
        attributes: src.attributes,
        text: src.text,
        children,
    }
}

fn relink_parent_pointers(node: &mut Node) {
    let self_ptr = Some(NonNull::from(&mut *node));
    for child in &mut node.children {
        child.parent = self_ptr;
        relink_parent_pointers(child);
    }
}

fn load_document_fixture(file_name: &str) -> Box<Node> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(file_name);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path.display(), e));
    let json: JsonNode =
        serde_json::from_str(&raw).unwrap_or_else(|e| panic!("Invalid fixture JSON: {}", e));
    let mut root = Box::new(node_from_fixture(json));
    relink_parent_pointers(root.as_mut());
    root
}

fn load_first_child_section(file_name: &str) -> Node {
    let root = load_document_fixture(file_name);
    assert_eq!(root.kind, ElementKind::Document);
    assert!(!root.children.is_empty(), "Fixture must contain one section child");
    let section = root.children[0].clone();
    assert_eq!(section.kind, ElementKind::Section);
    section
}

#[test]
fn validates_minimal_document_with_section_and_paragraph() {
    let tree = load_document_fixture("valid_document_section_paragraph.json");

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_invalid_list_child() {
    let tree = load_document_fixture("invalid_list_child.json");

    assert!(tree.validate().is_err());
}

#[test]
fn validates_figure_with_image_and_caption() {
    let tree = load_document_fixture("valid_figure_with_image_caption.json");

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_text_inside_empty_element() {
    let tree = load_document_fixture("invalid_meta_text.json");

    assert!(tree.validate().is_err());
}

#[test]
fn rejects_non_inline_child_in_paragraph() {
    let tree = load_document_fixture("invalid_non_inline_child.json");

    assert!(tree.validate().is_err());
}



#[test]
fn push_section_pushes_into_root_when_called_on_root() {
    let mut tree = load_document_fixture("document_root.json");
    let section = load_first_child_section("section_marker_hash_document.json");

    tree.push_section(section).unwrap();

    assert_eq!(tree.children.len(), 1);
    assert_eq!(tree.children[0].kind, ElementKind::Section);
}

#[test]
fn push_section_with_same_marker_pushes_to_parent_of_self() {
    let mut tree = load_document_fixture("section_single_hash.json");

    {
        let current_mut = &mut tree.children[0];
        let section = load_first_child_section("section_marker_hash_document.json");
        current_mut.push_section(section).unwrap();
    }

    assert_eq!(tree.children.len(), 2);
    assert_eq!(
        tree.children[1].attributes.get("section_marker").map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_with_ancestor_marker_pushes_to_parent_of_matching_ancestor() {
    let mut tree = load_document_fixture("section_nested_hash_tilde.json");

    {
        let inner_mut = &mut tree.children[0].children[0];
        let section = load_first_child_section("section_marker_hash_document.json");
        inner_mut.push_section(section).unwrap();
    }

    assert_eq!(tree.children.len(), 2);
    assert_eq!(
        tree.children[1].attributes.get("section_marker").map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_without_marker_match_pushes_to_closest_ancestor_section() {
    let mut tree = load_document_fixture("section_nested_hash_tilde.json");

    {
        let paragraph_mut = &mut tree.children[0].children[0].children[0];
        let section = load_first_child_section("section_marker_caret_document.json");
        paragraph_mut.push_section(section).unwrap();
    }

    let inner_section = &tree.children[0].children[0];
    assert_eq!(inner_section.children.len(), 2);
    assert_eq!(inner_section.children[1].kind, ElementKind::Section);
    assert_eq!(
        inner_section.children[1].attributes.get("section_marker").map(String::as_str),
        Some("^")
    );
}

#[test]
fn closest_ancestor_section_finds_nearest_section_upwards() {
    let tree = load_document_fixture("section_nested_hash_tilde.json");

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
    let tree = load_document_fixture("document_root.json");
    assert!(tree.closest_ancestor_section(None).is_none());
}

#[test]
fn closest_ancestor_section_matches_requested_marker() {
    let tree = load_document_fixture("section_nested_hash_tilde.json");

    let paragraph = &tree.children[0].children[0].children[0];
    let closest = paragraph.closest_ancestor_section(Some("#")).unwrap();

    assert_eq!(closest.kind, ElementKind::Section);
    assert_eq!(
        closest.attributes.get("section_marker").map(String::as_str),
        Some("#")
    );
}
