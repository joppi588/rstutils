// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;
#[cfg(test)]
mod tests;
mod validation;
pub use elements::{ContentModel, ElementCategory, ElementKind};
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};
use validation::allows_child;
pub use validation::ValidationError;

pub type NodeRef = Rc<RefCell<AstNode>>;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub kind: ElementKind,
    pub parent: Option<Weak<RefCell<AstNode>>>,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<NodeRef>,
}

impl AstNode {
    pub fn new_ref(kind: ElementKind) -> NodeRef {
        Rc::new(RefCell::new(Self {
            kind,
            parent: None,
            attributes: BTreeMap::new(),
            text: None,
            children: Vec::new(),
        }))
    }
    pub fn with_text(node_ref: &NodeRef, text: impl Into<String>) {
        node_ref.borrow_mut().text = Some(text.into());
    }

    pub fn with_attr(node_ref: &NodeRef, key: impl Into<String>, value: impl Into<String>) {
        node_ref
            .borrow_mut()
            .attributes
            .insert(key.into(), value.into());
    }

    pub fn push_child(parent: &NodeRef, child: NodeRef) -> Result<(), ValidationError> {
        let parent_kind = parent.borrow().kind;
        let child_kind = child.borrow().kind;
        if !allows_child(parent_kind, child_kind) {
            return Err(ValidationError::new(
                format!("invalid child {:?} inside {:?}", child_kind, parent_kind),
                Some(parent_kind),
                child_kind,
            ));
        }

        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
        Ok(())
    }

    pub fn push_section_ref(
        current: &NodeRef,
        section: NodeRef,
    ) -> Result<NodeRef, ValidationError> {
        let section_kind = section.borrow().kind;
        assert!(
            section_kind.has_category(ElementCategory::Structural),
            "push_section requires a structural node, got {:?}",
            section_kind
        );

        let section_marker = section.borrow().attributes.get("section_marker").cloned();

        let target_parent = if current.borrow().parent.is_none() {
            current.clone()
        } else {
            let self_marker = current.borrow().attributes.get("section_marker").cloned();
            if self_marker == section_marker {
                current
                    .borrow()
                    .parent
                    .as_ref()
                    .and_then(Weak::upgrade)
                    .expect("A section always has a parent.")
            } else if let Some(ancestor) =
                Self::closest_ancestor_section(current, section_marker.as_deref())
            {
                ancestor
                    .borrow()
                    .parent
                    .as_ref()
                    .and_then(Weak::upgrade)
                    .expect("A section always has a parent.")
            } else if let Some(closest) = Self::closest_ancestor_section(current, None) {
                closest
            } else {
                let mut root = current.clone();
                loop {
                    let next = {
                        let borrowed = root.borrow();
                        borrowed.parent.as_ref().and_then(Weak::upgrade)
                    };
                    match next {
                        Some(parent) => root = parent,
                        None => break,
                    }
                }
                root
            }
        };

        Self::push_child(&target_parent, section.clone())?;
        Ok(section)
    }

    pub fn closest_ancestor_section(
        node: &NodeRef,
        section_marker: Option<&str>,
    ) -> Option<NodeRef> {
        let mut current = node.borrow().parent.as_ref().and_then(Weak::upgrade);
        while let Some(current_node) = current.clone() {
            let matches = {
                let borrowed = current_node.borrow();
                borrowed.kind == ElementKind::Section
                    && section_marker.is_none_or(|marker| {
                        borrowed
                            .attributes
                            .get("section_marker")
                            .map(String::as_str)
                            == Some(marker)
                    })
            };
            if matches {
                return Some(current_node);
            }
            current = current_node
                .borrow()
                .parent
                .as_ref()
                .and_then(Weak::upgrade);
        }
        None
    }

    pub fn to_json(node_ref: &NodeRef) -> Value {
        let borrowed = node_ref.borrow();
        let mut attributes = Map::new();
        for (key, value) in &borrowed.attributes {
            attributes.insert(key.clone(), Value::String(value.clone()));
        }
        let children = borrowed
            .children
            .iter()
            .map(Self::to_json)
            .collect::<Vec<Value>>();

        let mut obj = Map::new();
        obj.insert(
            "kind".to_string(),
            Value::String(format!("{:?}", borrowed.kind)),
        );
        obj.insert("attributes".to_string(), Value::Object(attributes));
        obj.insert(
            "text".to_string(),
            borrowed
                .text
                .clone()
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        obj.insert("children".to_string(), Value::Array(children));
        Value::Object(obj)
    }

    pub fn to_yaml(node_ref: &NodeRef) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&Self::to_json(node_ref))
    }
}
