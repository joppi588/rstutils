// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{AstNode, ElementKind};
use serde::Deserialize;
use serde_json::json;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::rc::Rc;

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

fn node_from_fixture(src: FixtureNode) -> super::NodeRef {
    let node = AstNode::new_ref(src.kind);
    {
        let mut borrowed = node.borrow_mut();
        borrowed.attributes = src.attributes;
        borrowed.text = src.text;
    }

    for child_src in src.children {
        let child = node_from_fixture(child_src);
        child.borrow_mut().parent = Some(Rc::downgrade(&node));
        node.borrow_mut().children.push(child);
    }

    node
}

fn load_document_fixture(file_name: &str) -> super::NodeRef {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("data")
        .join(file_name);
    let raw = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to read fixture {}: {}", path.display(), e));
    let fixture: FixtureNode =
        serde_yaml::from_str(&raw).unwrap_or_else(|e| panic!("Invalid fixture YAML: {}", e));
    node_from_fixture(fixture)
}

fn section_with_marker(section_marker: &str) -> super::NodeRef {
    let section = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section, "section_marker", section_marker);
    section
}

fn validate_tree(node_ref: &super::NodeRef) -> Result<(), super::ValidationError> {
    let (parent_kind, children) = {
        let borrowed = node_ref.borrow();

        match borrowed.kind {
            ElementKind::Section => {
                if !matches!(
                    borrowed.children.first().map(|c| c.borrow().kind),
                    Some(ElementKind::Title)
                ) {
                    return Err(super::ValidationError {
                        message: "section must start with a title".to_string(),
                        parent: None,
                        node: borrowed.kind,
                    });
                }
            }
            ElementKind::Sidebar => {
                if matches!(
                    borrowed.children.first().map(|c| c.borrow().kind),
                    Some(ElementKind::Subtitle)
                ) {
                    return Err(super::ValidationError {
                        message: "sidebar subtitle requires a preceding title".to_string(),
                        parent: None,
                        node: borrowed.kind,
                    });
                }
            }
            ElementKind::Table => {
                if !borrowed
                    .children
                    .iter()
                    .any(|c| c.borrow().kind == ElementKind::Tgroup)
                {
                    return Err(super::ValidationError {
                        message: "table must contain a tgroup child".to_string(),
                        parent: None,
                        node: borrowed.kind,
                    });
                }
            }
            ElementKind::Tgroup => {
                if !borrowed
                    .children
                    .iter()
                    .any(|c| c.borrow().kind == ElementKind::Colspec)
                {
                    return Err(super::ValidationError {
                        message: "tgroup must contain at least one colspec child".to_string(),
                        parent: None,
                        node: borrowed.kind,
                    });
                }
            }
            _ => {}
        }

        match borrowed.kind.content_model() {
            super::ContentModel::Empty | super::ContentModel::ChildrenOnly => {
                if borrowed.text.is_some() {
                    return Err(super::ValidationError {
                        message: format!("{:?} must not carry text", borrowed.kind),
                        parent: borrowed
                            .parent
                            .as_ref()
                            .and_then(|p| p.upgrade())
                            .map(|p| p.borrow().kind),
                        node: borrowed.kind,
                    });
                }
            }
            super::ContentModel::TextOnly => {
                if !borrowed.children.is_empty() {
                    return Err(super::ValidationError {
                        message: format!("{:?} must not have children", borrowed.kind),
                        parent: borrowed
                            .parent
                            .as_ref()
                            .and_then(|p| p.upgrade())
                            .map(|p| p.borrow().kind),
                        node: borrowed.kind,
                    });
                }
            }
            super::ContentModel::TextOrInline => {}
        }

        (borrowed.kind, borrowed.children.clone())
    };

    for child in children {
        let child_kind = child.borrow().kind;
        if !super::allows_child(parent_kind, child_kind) {
            return Err(super::ValidationError {
                message: format!("invalid child {:?} inside {:?}", child_kind, parent_kind),
                parent: Some(parent_kind),
                node: child_kind,
            });
        }
        validate_tree(&child)?;
    }

    Ok(())
}

#[test]
fn validates_minimal_document_with_section_and_paragraph() {
    let tree = load_document_fixture("valid_document_section_paragraph.yaml");

    assert!(validate_tree(&tree).is_ok());
}

#[test]
fn rejects_invalid_list_child() {
    let tree = load_document_fixture("invalid_list_child.yaml");

    assert!(validate_tree(&tree).is_err());
}

#[test]
fn validates_figure_with_image_and_caption() {
    let tree = load_document_fixture("valid_figure_with_image_caption.yaml");

    assert!(validate_tree(&tree).is_ok());
}

#[test]
fn rejects_text_inside_empty_element() {
    let tree = load_document_fixture("invalid_meta_text.yaml");

    assert!(validate_tree(&tree).is_err());
}

#[test]
fn rejects_non_inline_child_in_paragraph() {
    let tree = load_document_fixture("invalid_non_inline_child.yaml");

    assert!(validate_tree(&tree).is_err());
}

#[test]
fn push_section_pushes_into_root_when_called_on_root() {
    let tree = load_document_fixture("document_root.yaml");
    let section = section_with_marker("#");

    AstNode::push_section_ref(&tree, section).unwrap();

    let borrowed = tree.borrow();
    assert_eq!(borrowed.children.len(), 1);
    assert_eq!(borrowed.children[0].borrow().kind, ElementKind::Section);
}

#[test]
fn push_section_with_same_marker_pushes_to_parent_of_self() {
    let tree = load_document_fixture("section_single_hash.yaml");

    let current = tree.borrow().children[0].clone();
    let section = section_with_marker("#");
    AstNode::push_section_ref(&current, section).unwrap();

    let borrowed = tree.borrow();
    assert_eq!(borrowed.children.len(), 2);
    assert_eq!(
        borrowed.children[1]
            .borrow()
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_with_ancestor_marker_pushes_to_parent_of_matching_ancestor() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let inner = tree.borrow().children[0].borrow().children[0].clone();
    let section = section_with_marker("#");
    AstNode::push_section_ref(&inner, section).unwrap();

    let borrowed = tree.borrow();
    assert_eq!(borrowed.children.len(), 2);
    assert_eq!(
        borrowed.children[1]
            .borrow()
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("#")
    );
}

#[test]
fn push_section_without_marker_match_pushes_to_closest_ancestor_section() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let paragraph = tree.borrow().children[0].borrow().children[0]
        .borrow()
        .children[0]
        .clone();
    let section = section_with_marker("^");
    AstNode::push_section_ref(&paragraph, section).unwrap();

    let inner_section = tree.borrow().children[0].borrow().children[0].clone();
    let inner_section_borrowed = inner_section.borrow();
    assert_eq!(inner_section_borrowed.children.len(), 2);
    assert_eq!(
        inner_section_borrowed.children[1].borrow().kind,
        ElementKind::Section
    );
    assert_eq!(
        inner_section_borrowed.children[1]
            .borrow()
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("^")
    );
}

#[test]
fn closest_ancestor_section_finds_nearest_section_upwards() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let paragraph = tree.borrow().children[0].borrow().children[0]
        .borrow()
        .children[0]
        .clone();
    let closest = AstNode::closest_ancestor_section(&paragraph, None).unwrap();
    let closest_borrowed = closest.borrow();

    assert_eq!(closest_borrowed.kind, ElementKind::Section);
    assert_eq!(
        closest_borrowed
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("~")
    );
}

#[test]
fn closest_ancestor_section_returns_none_at_root() {
    let tree = load_document_fixture("document_root.yaml");
    assert!(AstNode::closest_ancestor_section(&tree, None).is_none());
}

#[test]
fn closest_ancestor_section_matches_requested_marker() {
    let tree = load_document_fixture("section_nested_hash_tilde.yaml");

    let paragraph = tree.borrow().children[0].borrow().children[0]
        .borrow()
        .children[0]
        .clone();
    let closest = AstNode::closest_ancestor_section(&paragraph, Some("#")).unwrap();
    let closest_borrowed = closest.borrow();

    assert_eq!(closest_borrowed.kind, ElementKind::Section);
    assert_eq!(
        closest_borrowed
            .attributes
            .get("section_marker")
            .map(String::as_str),
        Some("#")
    );
}

#[test]
fn to_json_serializes_node_tree_without_parent() {
    let root = AstNode::new_ref(ElementKind::Document);
    AstNode::with_attr(&root, "lang", "rst");

    let section = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section, "opening_style", "=========");
    AstNode::with_attr(&section, "closing_style", "=========");

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title, "Heading 1\n");
    AstNode::push_child(&section, title).unwrap();
    AstNode::push_child(&root, section).unwrap();

    let json_value = AstNode::to_json(&root);

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
    let root = AstNode::new_ref(ElementKind::Document);
    AstNode::with_attr(&root, "lang", "rst");

    let section = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section, "opening_style", "=========");
    AstNode::with_attr(&section, "closing_style", "=========");

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title, "Heading 1\n");
    AstNode::push_child(&section, title).unwrap();
    AstNode::push_child(&root, section).unwrap();

    let yaml_text = AstNode::to_yaml(&root).expect("failed to serialize yaml");
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
