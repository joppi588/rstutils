// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod elements;
pub use elements::{ElementKind,ElementCategory,ContentModel};
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


    pub fn with_child(&mut self, child: Node) -> &mut Self {
        self.children.push(child);
        let self_ptr = Some(NonNull::from(&mut *self));
        if let Some(inserted) = self.children.last_mut() {
            inserted.parent = self_ptr;
            inserted.relink_descendant_parents();
        }
        self
    }

    pub fn push_child(&mut self, child: Node) -> Result<(), ValidationError> {
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
        self.children.push(child);
        let self_ptr = Some(NonNull::from(&mut *self));
        if let Some(inserted) = self.children.last_mut() {
            inserted.parent = self_ptr;
            inserted.relink_descendant_parents();
        }
        Ok(())
    }

    fn relink_descendant_parents(&mut self) {
        let self_ptr = Some(NonNull::from(&mut *self));
        for child in &mut self.children {
            child.parent = self_ptr;
            child.relink_descendant_parents();
        }
    }

    pub fn validate(&self) -> Result<(), ValidationError> {
        self.validate_with_parent(None)
    }


    pub fn push_section(&mut self, section: Node)-> Result<(), ValidationError>{

        let marker =self.attributes.get("section_marker").map(|x| x.as_str());

        match self.parent{
            None => self.push_child(section),
            _=>
            match self.match_section_stack(marker){
                Some(parent_node) => unsafe { (parent_node as *const Node as *mut Node).as_mut().unwrap().push_child(section) },
                None => unsafe { self.parent.unwrap().as_mut().push_child(section) }
            },
        
    }
    }

    pub fn match_section_stack(&self, section_marker: Option<&str>) -> Option<&Node>{
        let mut current = self.parent();
        let mut root = None;

        while let Some(node) = current {
            root = Some(node);
            if node.attributes.get("section_marker").map(|x| x.as_str()) == section_marker {
                return Some(node);
            }
            current = node.parent();
        }

        root
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
    fn parent_section_node_returns_parent_of_matching_section() {
        // GIVEN: An ast with three levels
        // WHEN: parent is queried
        // THEN: The right section is returned

        let mut inner_section = Node::new(ElementKind::Section).with_attr("section_marker", "~");
        inner_section.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

        let mut outer_section = Node::new(ElementKind::Section).with_attr("section_marker", "#");
        outer_section.with_child(inner_section);

        let mut tree = Node::new(ElementKind::Document);
        tree.with_child(outer_section);

        let current = &tree.children[0].children[0].children[0];
        let parent = current.match_section_stack(Some("#")).unwrap();

        assert_eq!(parent.kind, ElementKind::Section);
        assert_eq!(
            parent.attributes.get("section_marker").map(String::as_str),
            Some("#")
        );
    }

    #[test]
    fn parent_section_node_returns_root_if_no_matching_marker() {
        let mut section = Node::new(ElementKind::Section).with_attr("section_marker", "#");
        section.with_child(Node::new(ElementKind::Paragraph).with_text("Body text"));

        let mut tree = Node::new(ElementKind::Document);
        tree.with_child(section);

        let current = &tree.children[0].children[0];
        let parent = current.match_section_stack(Some("~")).unwrap();

        assert!(std::ptr::eq(parent, &tree));
    }
}
