// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;
pub use elements::{ElementKind,ElementCategory,ContentModel};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub kind: ElementKind,
    pub parent: Option<ElementKind>,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(kind: ElementKind) -> Self {
        Self {
            kind,
            parent: None,
            attributes: BTreeMap::new(),
            text: None,
            children: Vec::new(),
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }


    // TODO: eventually remove, used only in tests
    pub fn with_child(mut self, mut child: Node) -> Self {
        child.parent = Some(self.kind);
        self.children.push(child);
        self
    }

    pub fn push_child(&mut self, mut child: Node) -> Result<(), ValidationError> {
        if !allows_child(self.kind, child.kind) {
            return Err(ValidationError::new(
                format!(
                    "invalid child {:?} inside {:?}",
                    child.kind,
                    self.kind
                ),
                Some(self.kind),
                child.kind,
            ));
        }
        child.parent = Some(self.kind);
        self.children.push(child);
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.validate_with_parent(None)
    }

    fn validate_with_parent(&self, parent: Option<ElementKind>) -> Result<(), ValidationError> {
        if let Some(parent_kind) = parent {
            if !allows_child(parent_kind, self.kind) {
                return Err(ValidationError::new(
                    format!(
                        "invalid child {:?} inside {:?}",
                        self.kind,
                        parent_kind
                    ),
                    Some(parent_kind),
                    self.kind,
                ));
            }
        }

        match self.kind.content_model() {
            ContentModel::Empty => {
                if self.text.as_deref().is_some_and(|t| !t.is_empty()) || !self.children.is_empty() {
                    return Err(ValidationError::new(
                        "empty element must not contain text or children",
                        parent,
                        self.kind,
                    ));
                }
            }
            ContentModel::TextOnly => {
                if !self.children.is_empty() {
                    return Err(ValidationError::new(
                        "text-only element must not contain children",
                        parent,
                        self.kind,
                    ));
                }
            }
            ContentModel::TextOrInline => {
                for child in &self.children {
                    if !child.kind.has_category(ElementCategory::Inline) {
                        return Err(ValidationError::new(
                            format!("non-inline child {:?} in text-or-inline element", child.kind),
                            parent,
                            self.kind,
                        ));
                    }
                }
            }
            ContentModel::ChildrenOnly => {
                if self.text.as_deref().is_some_and(|t| !t.is_empty()) {
                    return Err(ValidationError::new(
                        "children-only element must not contain text",
                        parent,
                        self.kind,
                    ));
                }
            }
        }

        validate_element_shape(self)?;

        for child in &self.children {
            child.validate_with_parent(Some(self.kind))?;
        }

        Ok(())
    }
}

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
                self.message,
                self.node,
                parent
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

        Admonition => {
            is_any_of(child, &[Title]) || child.has_category(ElementCategory::Body)
        }

        BlockQuote
        | Compound
        | Container
        | Definition
        | Description
        | Entry
        | FieldBody
        | Footer
        | Footnote
        | Header
        | Hint
        | Important
        | Legend
        | ListItem
        | Note
        | SystemMessage
        | Tip
        | Warning
        | Attention
        | Caution
        | Danger
        | Error
        | Citation => {
            child.has_category(ElementCategory::Body)
                || is_any_of(
                    child,
                    &[
                        Attribution,
                        Label,
                        Title,
                    ],
                )
        }

        BulletList | EnumeratedList => is_any_of(child, &[ListItem]),

        DefinitionList => is_any_of(child, &[DefinitionListItem]),

        DefinitionListItem => {
            is_any_of(child, &[Term, Classifier, Definition])
        }

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

fn validate_element_shape(node: &Node) -> Result<(), ValidationError> {
    use ElementKind::*;

    match node.kind {
        Section => {
            if !matches!(node.children.first().map(|c| c.kind), Some(Title)) {
                return Err(ValidationError::new(
                    "section must start with a title",
                    None,
                    node.kind,
                ));
            }
        }
        Sidebar => {
            if matches!(node.children.first().map(|c| c.kind), Some(Subtitle)) {
                return Err(ValidationError::new(
                    "sidebar subtitle requires a preceding title",
                    None,
                    node.kind,
                ));
            }
        }
        Table => {
            if !node.children.iter().any(|c| c.kind == Tgroup) {
                return Err(ValidationError::new(
                    "table must contain a tgroup child",
                    None,
                    node.kind,
                ));
            }
        }
        Tgroup => {
            if !node.children.iter().any(|c| c.kind == Colspec) {
                return Err(ValidationError::new(
                    "tgroup must contain at least one colspec child",
                    None,
                    node.kind,
                ));
            }
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ElementKind, Node};

    #[test]
    fn validates_minimal_document_with_section_and_paragraph() {
        let tree = Node::new(ElementKind::Document).with_child(
            Node::new(ElementKind::Section)
                .with_child(Node::new(ElementKind::Title).with_text("Heading"))
                .with_child(Node::new(ElementKind::Paragraph).with_text("Body text")),
        );

        assert!(tree.validate().is_ok());
    }

    #[test]
    fn rejects_invalid_list_child() {
        let tree = Node::new(ElementKind::Document).with_child(
            Node::new(ElementKind::BulletList)
                .with_child(Node::new(ElementKind::Paragraph).with_text("not a list item")),
        );

        assert!(tree.validate().is_err());
    }

    #[test]
    fn validates_figure_with_image_and_caption() {
        let tree = Node::new(ElementKind::Document).with_child(
            Node::new(ElementKind::Figure)
                .with_child(Node::new(ElementKind::Image))
                .with_child(Node::new(ElementKind::Caption).with_text("Figure caption")),
        );

        assert!(tree.validate().is_ok());
    }

    #[test]
    fn rejects_text_inside_empty_element() {
        let tree = Node::new(ElementKind::Document)
            .with_child(Node::new(ElementKind::Meta).with_text("x"));

        assert!(tree.validate().is_err());
    }

    #[test]
    fn rejects_non_inline_child_in_paragraph() {
        let tree = Node::new(ElementKind::Document).with_child(
            Node::new(ElementKind::Paragraph)
                .with_child(Node::new(ElementKind::BulletList).with_child(Node::new(ElementKind::ListItem))),
        );

        assert!(tree.validate().is_err());
    }
}
