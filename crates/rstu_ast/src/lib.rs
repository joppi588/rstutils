// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;
#[cfg(test)]
mod tests;
pub use elements::{ContentModel, ElementCategory, ElementKind};
use serde_json::{Map, Value};
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
    pub parent: Option<ElementKind>,
    pub node: ElementKind,
}

impl ValidationError {
    fn new(message: impl Into<String>, parent: Option<ElementKind>, node: ElementKind) -> Self {
        Self {
            message: message.into(),
            parent,
            node,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(parent) = self.parent {
            write!(
                f,
                "{} (node: {:?}, parent: {:?})",
                self.message, self.node, parent
            )
        } else {
            write!(f, "{} (node: {:?})", self.message, self.node)
        }
    }
}

impl Error for ValidationError {}

fn is_any_of<const N: usize>(kind: ElementKind, allowed: &[ElementKind; N]) -> bool {
    allowed.contains(&kind)
}

fn allows_child(parent: ElementKind, child: ElementKind) -> bool {
    use ElementKind::*;

    match parent {
        Document => {
            child.has_category(ElementCategory::Structural)
                || child.has_category(ElementCategory::StructuralSub)
                || child.has_category(ElementCategory::Body)
        }

        Section => {
            child.has_category(ElementCategory::Structural)
                || child.has_category(ElementCategory::StructuralSub)
                || child.has_category(ElementCategory::Body)
        }

        Sidebar => {
            is_any_of(child, &[Title, Subtitle, Topic]) || child.has_category(ElementCategory::Body)
        }

        Topic => is_any_of(child, &[Title]) || child.has_category(ElementCategory::Body),

        Decoration => is_any_of(child, &[Header, Footer]),
        Docinfo => child.has_category(ElementCategory::Bibliographic),

        Authors => is_any_of(child, &[Author, Organization, Address, Contact]),

        Admonition => is_any_of(child, &[Title]) || child.has_category(ElementCategory::Body),

        BlockQuote | Compound | Container | Definition | Description | Entry | FieldBody
        | Footer | Footnote | Header | Hint | Important | Legend | ListItem | Note
        | SystemMessage | Tip | Warning | Attention | Caution | Danger | Error | Citation => {
            child.has_category(ElementCategory::Body)
                || is_any_of(child, &[Attribution, Label, Title])
        }

        BulletList | EnumeratedList => is_any_of(child, &[ListItem]),

        DefinitionList => is_any_of(child, &[DefinitionListItem]),

        DefinitionListItem => is_any_of(child, &[Term, Classifier, Definition]),

        FieldList => is_any_of(child, &[Field]),
        Field => is_any_of(child, &[FieldName, FieldBody]),

        Figure => is_any_of(child, &[Image, Reference, Caption, Legend]),

        LineBlock => is_any_of(child, &[Line, LineBlock]),

        OptionList => is_any_of(child, &[OptionListItem]),
        OptionListItem => is_any_of(child, &[OptionGroup, Description]),
        OptionGroup => is_any_of(child, &[Option]),
        Option => is_any_of(child, &[OptionString, OptionArgument]),

        Table => is_any_of(child, &[Title, Tgroup]),
        Tgroup => is_any_of(child, &[Colspec, Thead, Tbody]),
        Thead | Tbody => is_any_of(child, &[Row]),
        Row => is_any_of(child, &[Entry]),

        Paragraph
        | Rubric
        | Title
        | Subtitle
        | Emphasis
        | Strong
        | Literal
        | Reference
        | Inline
        | Subscript
        | Superscript
        | Abbreviation
        | Acronym
        | Generated
        | Problematic
        | SubstitutionReference
        | SubstitutionDefinition
        | TitleReference
        | Address
        | Author
        | Contact
        | Copyright
        | Date
        | DoctestBlock
        | FieldName
        | Line
        | LiteralBlock
        | Organization
        | Revision
        | Status
        | Term
        | Version => child.has_category(ElementCategory::Inline),

        _ => false,
    }
}

// fn validate_element_shape(node: &AstNode) -> Result<(), ValidationError> {
//     use ElementKind::*;

//     match node.kind {
//         Section => {
//             if !matches!(node.children.first().map(|c| c.borrow().kind), Some(Title)) {
//                 return Err(ValidationError::new(
//                     "section must start with a title",
//                     None,
//                     node.kind,
//                 ));
//             }
//         }
//         Sidebar => {
//             if matches!(node.children.first().map(|c| c.borrow().kind), Some(Subtitle)) {
//                 return Err(ValidationError::new(
//                     "sidebar subtitle requires a preceding title",
//                     None,
//                     node.kind,
//                 ));
//             }
//         }
//         Table => {
//             if !node.children.iter().any(|c| c.borrow().kind == Tgroup) {
//                 return Err(ValidationError::new(
//                     "table must contain a tgroup child",
//                     None,
//                     node.kind,
//                 ));
//             }
//         }
//         Tgroup => {
//             if !node.children.iter().any(|c| c.borrow().kind == Colspec) {
//                 return Err(ValidationError::new(
//                     "tgroup must contain at least one colspec child",
//                     None,
//                     node.kind,
//                 ));
//             }
//         }
//         _ => {}
//     }

//     Ok(())
// }

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
