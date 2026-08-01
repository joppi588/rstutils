// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::elements::{ElementCategory, ElementKind};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub message: String,
    pub parent: Option<ElementKind>,
    pub node: ElementKind,
}

impl ValidationError {
    pub(super) fn new(
        message: impl Into<String>,
        parent: Option<ElementKind>,
        node: ElementKind,
    ) -> Self {
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

#[cfg(test)]
pub(crate) fn _validate_tree(node_ref: &super::NodeRef) -> Result<(), super::ValidationError> {
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
        _validate_tree(&child)?;
    }

    Ok(())
}

fn is_any_of<const N: usize>(kind: ElementKind, allowed: &[ElementKind; N]) -> bool {
    allowed.contains(&kind)
}

pub(super) fn allows_child(parent: ElementKind, child: ElementKind) -> bool {
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

        Block => is_any_of(child, &[Paragraph]),

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
