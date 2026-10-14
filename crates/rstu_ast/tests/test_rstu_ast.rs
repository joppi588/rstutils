// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::AstNode;
use rstu_ast::{NodeClass, NodeRefExt};
use std::rc::Rc;

fn section_with_marker(section_marker: &str) -> rstu_ast::NodeRef {
    let section = AstNode::new_ref(NodeClass::Section);
    section.with_attr("section_marker", section_marker);
    section
}

#[test]
/// GIVEN attributes containing string and integer values
/// WHEN their values are accessed by name
/// THEN as_str returns the string value only for the string attribute
fn attributes_index_by_name_and_convert_strings() {
    let node = AstNode::new_ref(NodeClass::Document);
    node.with_attr("name", "rst").with_attr("count", 1usize);

    let borrowed = node.borrow();
    assert_eq!(
        borrowed.attributes["name"].as_str(),
        Some("rst".to_string())
    );
    assert_eq!(borrowed.attributes["count"].as_str(), Some("1".to_string()));
}

#[test]
/// GIVEN a parent node and a child node
/// WHEN the child is pushed to the parent
/// THEN the child is appended and its parent is set
fn push_child_sets_parent_and_appends_child() {
    let parent = AstNode::new_ref(NodeClass::Document);
    let child = AstNode::new_ref(NodeClass::Paragraph);
    let child_ref_cloned = child.clone(); // cloning the ref, not the node!

    parent.push_child(child);

    let borrowed = parent.borrow();
    assert_eq!(borrowed.children.len(), 1);
    assert!(Rc::ptr_eq(&borrowed.children[0], &child_ref_cloned));
    assert!(Rc::ptr_eq(&child_ref_cloned.get_parent().unwrap(), &parent));
}

#[test]
/// GIVEN a section attached to a tree
/// WHEN a section with the same marker is pushed from it
/// THEN the new section is appended to the section's parent
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
/// GIVEN nested sections with different markers and a paragraph in the inner section
/// WHEN searching for an ancestor section with the outer marker
/// THEN the outer section is returned
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
