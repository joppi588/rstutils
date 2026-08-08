// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod attr;
mod nodes;
#[cfg(test)]
mod tests;
pub use attr::AttributeType;
pub use nodes::NodeClass;
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};

pub type NodeRef = Rc<RefCell<AstNode>>;

#[derive(Debug, Clone)]
pub struct AstNode {
    pub class: NodeClass,
    pub parent: Option<Weak<RefCell<AstNode>>>,
    pub attributes: BTreeMap<String, AttributeType>,
    pub text: Option<String>,
    pub children: Vec<NodeRef>,
}

pub trait NodeRefExt {
    fn with_text(&self, text: impl Into<String>) -> NodeRef;
    fn with_attr(&self, key: impl Into<String>, value: impl Into<AttributeType>) -> NodeRef;
}

impl NodeRefExt for NodeRef {
    fn with_text(&self, text: impl Into<String>) -> NodeRef {
        self.borrow_mut().text = Some(text.into());
        self.clone()
    }

    fn with_attr(&self, key: impl Into<String>, value: impl Into<AttributeType>) -> NodeRef {
        self.borrow_mut()
            .attributes
            .insert(key.into(), value.into());
        self.clone()
    }
}

impl AstNode {
    pub fn new_ref(class: NodeClass) -> NodeRef {
        Rc::new(RefCell::new(Self {
            class,
            parent: None,
            attributes: BTreeMap::new(),
            text: None,
            children: Vec::new(),
        }))
    }

    pub fn push_child(parent: &NodeRef, child: NodeRef) {
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
    }

    pub fn push_body_element(current: &NodeRef, body: NodeRef) -> NodeRef {
        if matches!(
            current.borrow().class,
            NodeClass::Document | NodeClass::Section
        ) {
            Self::push_child(current, body.clone());
        } else {
            let parent = current
                .borrow()
                .parent
                .as_ref()
                .and_then(Weak::upgrade)
                .unwrap_or_else(|| current.clone());
            Self::push_child(&parent, body.clone());
        }

        body
    }

    pub fn push_section_ref(current: &NodeRef, section: NodeRef) -> NodeRef {
        assert_eq!(
            section.borrow().class,
            NodeClass::Section,
            "push_section_ref requires a section node"
        );

        let section_marker = section
            .borrow()
            .attributes
            .get("section_marker")
            .and_then(AttributeType::as_str)
            .map(str::to_owned);

        let target_parent = if current.borrow().parent.is_none() {
            current.clone()
        } else {
            let self_marker = current
                .borrow()
                .attributes
                .get("section_marker")
                .and_then(AttributeType::as_str)
                .map(str::to_owned);
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

        Self::push_child(&target_parent, section.clone());
        section
    }

    /// returns the current section the node is in
    /// - with the given marker
    /// - the lowest section if no marker given.
    pub fn closest_ancestor_section(
        node: &NodeRef,
        section_marker: Option<&str>,
    ) -> Option<NodeRef> {
        let mut current = Some(node.clone());
        while let Some(current_node) = current.clone() {
            let matches = {
                let borrowed = current_node.borrow();
                borrowed.class == NodeClass::Section
                    && section_marker.is_none_or(|marker| {
                        borrowed
                            .attributes
                            .get("section_marker")
                            .and_then(AttributeType::as_str)
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
            attributes.insert(key.clone(), value.to_json_value());
        }
        let children = borrowed
            .children
            .iter()
            .map(Self::to_json)
            .collect::<Vec<Value>>();

        let mut obj = Map::new();
        obj.insert(
            "class".to_string(),
            Value::String(format!("{:?}", borrowed.class)),
        );
        if attributes.len() > 0 {
            obj.insert("attributes".to_string(), Value::Object(attributes));
        };
        if let Some(v) = borrowed.text.clone().map(Value::String) {
            obj.insert("text".to_string(), v);
        };
        if children.len() > 0 {
            obj.insert("children".to_string(), Value::Array(children));
        };
        Value::Object(obj)
    }

    pub fn to_yaml(node_ref: &NodeRef) -> Result<String, serde_yaml::Error> {
        serde_yaml::to_string(&Self::to_json(node_ref))
    }
}
