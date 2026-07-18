// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT


use super::{ElementKind, Node};

#[test]
fn validates_minimal_document_with_section_and_paragraph() {
    let mut section = Node::new(ElementKind::Section);
    section.with_child(Node::new(ElementKind::Title).with_text("Heading"));
    section.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(section);

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_invalid_list_child() {
    let mut bullet_list = Node::new(ElementKind::BulletList);
    bullet_list.with_child(Node::new(ElementKind::Paragraph).with_text("not a list item"));

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(bullet_list);

    assert!(tree.validate().is_err());
}

#[test]
fn validates_figure_with_image_and_caption() {
    let mut figure = Node::new(ElementKind::Figure);
    figure.with_child(Node::new(ElementKind::Image));
    figure.with_child(Node::new(ElementKind::Caption).with_text("Figure caption"));

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(figure);

    assert!(tree.validate().is_ok());
}

#[test]
fn rejects_text_inside_empty_element() {
    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(Node::new(ElementKind::Meta).with_text("x"));

    assert!(tree.validate().is_err());
}

#[test]
fn rejects_non_inline_child_in_paragraph() {
    let mut bullet_list = Node::new(ElementKind::BulletList);
    bullet_list.with_child(Node::new(ElementKind::ListItem));

    let mut paragraph = Node::new(ElementKind::Paragraph);
    paragraph.with_child(bullet_list);

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(paragraph);

    assert!(tree.validate().is_err());
}



#[test]
fn push_section_pushes_into_root_when_called_on_root() {
    let mut tree = Node::new(ElementKind::Document);
    let section = Node::new(ElementKind::Section).with_attr("section_marker", "#");

    tree.push_section(section).unwrap();

    assert_eq!(tree.children.len(), 1);
    assert_eq!(tree.children[0].kind, ElementKind::Section);
}

#[test]
fn push_section_with_same_marker_pushes_to_parent_of_self() {
    let mut current = Node::new(ElementKind::Section).with_attr("section_marker", "#");
    current.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(current);

    {
        let current_mut = &mut tree.children[0];
        let section = Node::new(ElementKind::Section).with_attr("section_marker", "#");
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
    let mut inner = Node::new(ElementKind::Section).with_attr("section_marker", "~");
    inner.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

    let mut outer = Node::new(ElementKind::Section).with_attr("section_marker", "#");
    outer.with_child(inner);

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(outer);

    {
        let inner_mut = &mut tree.children[0].children[0];
        let section = Node::new(ElementKind::Section).with_attr("section_marker", "#");
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
    let mut inner = Node::new(ElementKind::Section).with_attr("section_marker", "~");
    inner.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

    let mut outer = Node::new(ElementKind::Section).with_attr("section_marker", "#");
    outer.with_child(inner);

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(outer);

    {
        let paragraph_mut = &mut tree.children[0].children[0].children[0];
        let section = Node::new(ElementKind::Section).with_attr("section_marker", "^");
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
    let mut inner = Node::new(ElementKind::Section).with_attr("section_marker", "~");
    inner.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

    let mut outer = Node::new(ElementKind::Section).with_attr("section_marker", "#");
    outer.with_child(inner);

    let mut tree = Node::new(ElementKind::Document);
    tree.with_child(outer);

    let paragraph = &tree.children[0].children[0].children[0];
    let closest = paragraph.closest_ancestor_section().unwrap();

    assert_eq!(closest.kind, ElementKind::Section);
    assert_eq!(
        closest.attributes.get("section_marker").map(String::as_str),
        Some("~")
    );
}

#[test]
fn closest_ancestor_section_returns_none_at_root() {
    let tree = Node::new(ElementKind::Document);
    assert!(tree.closest_ancestor_section().is_none());
}
