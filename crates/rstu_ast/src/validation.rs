// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

// Note: Currently not in active use, keep for later

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

fn validate_element_shape(node: &AstNode) -> Result<(), ValidationError> {
    use ElementKind::*;

    match node.kind {
        Section => {
            if !matches!(node.children.first().map(|c| c.borrow().kind), Some(Title)) {
                return Err(ValidationError::new(
                    "section must start with a title",
                    None,
                    node.kind,
                ));
            }
        }
        Sidebar => {
            if matches!(
                node.children.first().map(|c| c.borrow().kind),
                Some(Subtitle)
            ) {
                return Err(ValidationError::new(
                    "sidebar subtitle requires a preceding title",
                    None,
                    node.kind,
                ));
            }
        }
        Table => {
            if !node.children.iter().any(|c| c.borrow().kind == Tgroup) {
                return Err(ValidationError::new(
                    "table must contain a tgroup child",
                    None,
                    node.kind,
                ));
            }
        }
        Tgroup => {
            if !node.children.iter().any(|c| c.borrow().kind == Colspec) {
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
