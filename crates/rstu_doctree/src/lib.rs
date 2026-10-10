// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;

mod validation;
pub use elements::{ContentModel, ElementCategory, ElementKind};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::{Rc, Weak};
pub use validation::ValidationError;

pub type NodeRef = Rc<RefCell<DocTreeNode>>;

#[derive(Debug, Clone)]
pub struct DocTreeNode {
    pub kind: ElementKind,
    pub parent: Option<Weak<RefCell<DocTreeNode>>>,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<NodeRef>,
}

impl DocTreeNode {
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
        let _parent_kind = parent.borrow().kind;
        let _child_kind = child.borrow().kind;

        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push(child);
        Ok(())
    }
}
