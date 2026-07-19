// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;
#[cfg(test)]
mod tests;
pub use elements::{ContentModel, ElementCategory, ElementKind};
use std::collections::BTreeMap;
use std::error::Error;
use std::fmt;
use std::ptr::NonNull;

#[derive(Debug)]
pub struct Node {
    pub kind: ElementKind,
    /// Raw pointer to the parent node.
    /// Safety: valid only as long as the parent node has not been moved in memory.
    pub parent: Option<NonNull<Node>>,
    pub attributes: BTreeMap<String, String>,
    pub text: Option<String>,
    pub children: Vec<Node>,
}

// Safety: Node does not use interior mutability through the raw pointer.
// The pointer is a back-reference only; ownership follows the tree structure.
unsafe impl Send for Node {}
unsafe impl Sync for Node {}

impl Clone for Node {
    /// Clones the subtree. The clone's `parent` is reset to `None` since it is
    /// detached from the original tree.
    fn clone(&self) -> Self {
        Node {
            kind: self.kind,
            parent: None,
            attributes: self.attributes.clone(),
            text: self.text.clone(),
            children: self.children.clone(),
        }
    }
}

impl PartialEq for Node {
    /// Structural equality; the `parent` pointer is intentionally excluded.
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
            && self.attributes == other.attributes
            && self.text == other.text
            && self.children == other.children
    }
}

impl Eq for Node {}

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

    /// Returns a shared reference to the parent node, if any.
    ///
    /// # Safety
    /// The reference is valid only as long as the parent node has not been
    /// moved in memory since the pointer was set.
    pub fn parent(&self) -> Option<&Node> {
        // SAFETY: caller upholds the invariant that the parent is still live.
        self.parent.map(|p| unsafe { p.as_ref() })
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    // TODO: Eventually remove
    pub fn with_child(&mut self, child: Node) -> &mut Self {
        self.children.push(child);
        let self_ptr = Some(NonNull::from(&mut *self));
        if let Some(inserted) = self.children.last_mut() {
            inserted.parent = self_ptr;
            relink_parent_pointers(inserted);
        }
        self
    }

    pub fn push_child(&mut self, child: Node) -> Result<(), ValidationError> {
        if !allows_child(self.kind, child.kind) {
            return Err(ValidationError::new(
                format!("invalid child {:?} inside {:?}", child.kind, self.kind),
                Some(self.kind),
                child.kind,
            ));
        }
        self.children.push(child);
        let self_ptr = Some(NonNull::from(&mut *self));
        if let Some(inserted) = self.children.last_mut() {
            inserted.parent = self_ptr;
            relink_parent_pointers(inserted);
        }
        Ok(())
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.validate_with_parent(None)
    }

    pub fn closest_ancestor_section(&self, section_marker: Option<&str>) -> Option<&Node> {
        let mut current = self.parent();
        while let Some(node) = current {
            if node.kind == ElementKind::Section
                && section_marker.is_none_or(|marker| {
                    node.attributes.get("section_marker").map(String::as_str) == Some(marker)
                })
            {
                return Some(node);
            }
            current = node.parent();
        }
        None
    }

    pub fn push_section(&mut self, section: Node) -> Result<(), ValidationError> {
        assert!(
            section.kind.has_category(ElementCategory::Structural),
            "push_section requires a structural node, got {:?}",
            section.kind
        );

        let section_marker = section
            .attributes
            .get("section_marker")
            .map(String::as_str)
            .map(str::to_owned);

        if self.parent.is_none() {
            return self.push_child(section);
        }

        let self_marker = self.attributes.get("section_marker").map(String::as_str);
        if self_marker == section_marker.as_deref() {
            let mut parent = self.parent.expect("A section always has a parent.");
            return unsafe { parent.as_mut().push_child(section) };
        }

        if let Some(node) = self.closest_ancestor_section(section_marker.as_deref()) {
            let mut parent = node.parent.expect("A section always has a parent.");
            return unsafe { parent.as_mut().push_child(section) };
        }

        if let Some(closest_section) = self.closest_ancestor_section(None) {
            return unsafe {
                (closest_section as *const Node as *mut Node)
                    .as_mut()
                    .unwrap()
                    .push_child(section)
            };
        }

        let mut root = self.parent.expect("A section always has a parent.");
        unsafe {
            while let Some(parent) = root.as_ref().parent {
                root = parent;
            }
            root.as_mut().push_child(section)
        }
    }

    fn validate_with_parent(&self, parent: Option<ElementKind>) -> Result<(), ValidationError> {
        if let Some(parent_kind) = parent {
            if !allows_child(parent_kind, self.kind) {
                return Err(ValidationError::new(
                    format!("invalid child {:?} inside {:?}", self.kind, parent_kind),
                    Some(parent_kind),
                    self.kind,
                ));
            }
        }

        match self.kind.content_model() {
            ContentModel::Empty => {
                if self.text.as_deref().is_some_and(|t| !t.is_empty()) || !self.children.is_empty()
                {
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
                            format!(
                                "non-inline child {:?} in text-or-inline element",
                                child.kind
                            ),
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

pub(crate) fn relink_parent_pointers(node: &mut Node) {
    let self_ptr = Some(NonNull::from(&mut *node));
    for child in &mut node.children {
        child.parent = self_ptr;
        relink_parent_pointers(child);
    }
}
