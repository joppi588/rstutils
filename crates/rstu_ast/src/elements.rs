// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentModel {
    Empty,
    TextOnly,
    TextOrInline,
    ChildrenOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementCategory {
    Root,
    Structural,
    StructuralSub,
    Decorative,
    Bibliographic,
    Body,
    BodySub,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum ElementKind {
    Abbreviation,
    Acronym,
    Address,
    Admonition,
    Attention,
    Attribution,
    Author,
    Authors,
    BlockQuote,
    BulletList,
    Caption,
    Caution,
    Citation,
    CitationReference,
    Classifier,
    Colspec,
    Comment,
    Compound,
    Contact,
    Container,
    Copyright,
    Danger,
    Date,
    Decoration,
    Definition,
    DefinitionList,
    DefinitionListItem,
    Description,
    Docinfo,
    DoctestBlock,
    Document,
    Emphasis,
    Entry,
    EnumeratedList,
    Error,
    Field,
    FieldBody,
    FieldList,
    FieldName,
    Figure,
    Footer,
    Footnote,
    FootnoteReference,
    Generated,
    Header,
    Hint,
    Image,
    Important,
    Inline,
    Label,
    Legend,
    Line,
    LineBlock,
    ListItem,
    Literal,
    LiteralBlock,
    Math,
    MathBlock,
    Meta,
    Note,
    Option,
    OptionArgument,
    OptionGroup,
    OptionList,
    OptionListItem,
    OptionString,
    Organization,
    Paragraph,
    Pending,
    Problematic,
    Raw,
    Reference,
    Revision,
    Row,
    Rubric,
    Section,
    Sidebar,
    Status,
    Strong,
    Subscript,
    SubstitutionDefinition,
    SubstitutionReference,
    Subtitle,
    Superscript,
    SystemMessage,
    Table,
    Target,
    Tbody,
    Term,
    Tgroup,
    Thead,
    Tip,
    Title,
    TitleReference,
    Topic,
    Transition,
    Version,
    Warning,
}

impl ElementKind {
    pub fn categories(self) -> Vec<ElementCategory> {
        use ElementCategory::*;

        match self {
            ElementKind::Field => vec![Bibliographic, BodySub],
            ElementKind::Image => vec![Body, Inline],
            ElementKind::Raw => vec![Body, Inline],
            ElementKind::Reference => vec![Body, Inline],
            ElementKind::Target => vec![Body, Inline],
            ElementKind::Title => vec![StructuralSub, BodySub],
            ElementKind::Document => vec![Root],
            ElementKind::Section | ElementKind::Sidebar | ElementKind::Topic => vec![Structural],
            ElementKind::Meta
            | ElementKind::Transition
            | ElementKind::Subtitle
            | ElementKind::Decoration
            | ElementKind::Docinfo => vec![StructuralSub],
            ElementKind::Header | ElementKind::Footer => vec![Decorative],
            ElementKind::Address
            | ElementKind::Author
            | ElementKind::Authors
            | ElementKind::Contact
            | ElementKind::Copyright
            | ElementKind::Date
            | ElementKind::Organization
            | ElementKind::Revision
            | ElementKind::Status
            | ElementKind::Version => vec![Bibliographic],
            ElementKind::Attribution
            | ElementKind::Caption
            | ElementKind::Classifier
            | ElementKind::Colspec
            | ElementKind::Definition
            | ElementKind::DefinitionListItem
            | ElementKind::Description
            | ElementKind::Entry
            | ElementKind::FieldBody
            | ElementKind::FieldName
            | ElementKind::Label
            | ElementKind::Legend
            | ElementKind::Line
            | ElementKind::ListItem
            | ElementKind::Option
            | ElementKind::OptionArgument
            | ElementKind::OptionGroup
            | ElementKind::OptionListItem
            | ElementKind::OptionString
            | ElementKind::Row
            | ElementKind::Tbody
            | ElementKind::Term
            | ElementKind::Tgroup
            | ElementKind::Thead => vec![BodySub],
            ElementKind::Abbreviation
            | ElementKind::Acronym
            | ElementKind::CitationReference
            | ElementKind::Emphasis
            | ElementKind::FootnoteReference
            | ElementKind::Generated
            | ElementKind::Inline
            | ElementKind::Literal
            | ElementKind::Math
            | ElementKind::Problematic
            | ElementKind::Strong
            | ElementKind::Subscript
            | ElementKind::SubstitutionReference
            | ElementKind::Superscript
            | ElementKind::TitleReference => vec![Inline],
            ElementKind::Pending
            | ElementKind::Comment
            | ElementKind::DoctestBlock
            | ElementKind::LiteralBlock
            | ElementKind::MathBlock
            | ElementKind::Paragraph
            | ElementKind::Rubric
            | ElementKind::SubstitutionDefinition
            | ElementKind::Admonition
            | ElementKind::Attention
            | ElementKind::BlockQuote
            | ElementKind::BulletList
            | ElementKind::Caution
            | ElementKind::Citation
            | ElementKind::Compound
            | ElementKind::Container
            | ElementKind::Danger
            | ElementKind::DefinitionList
            | ElementKind::EnumeratedList
            | ElementKind::Error
            | ElementKind::FieldList
            | ElementKind::Figure
            | ElementKind::Footnote
            | ElementKind::Hint
            | ElementKind::Important
            | ElementKind::LineBlock
            | ElementKind::Note
            | ElementKind::OptionList
            | ElementKind::SystemMessage
            | ElementKind::Table
            | ElementKind::Tip
            | ElementKind::Warning => vec![Body],
        }
    }

    pub fn content_model(self) -> ContentModel {
        match self {
            ElementKind::Meta
            | ElementKind::Transition
            | ElementKind::Colspec
            | ElementKind::Image
            | ElementKind::Pending => ContentModel::Empty,

            ElementKind::CitationReference
            | ElementKind::FootnoteReference
            | ElementKind::Label
            | ElementKind::Math
            | ElementKind::OptionArgument
            | ElementKind::OptionString
            | ElementKind::Raw
            | ElementKind::Comment
            | ElementKind::Target => ContentModel::TextOnly,

            ElementKind::Abbreviation
            | ElementKind::Acronym
            | ElementKind::Address
            | ElementKind::Attribution
            | ElementKind::Author
            | ElementKind::Caption
            | ElementKind::Classifier
            | ElementKind::Contact
            | ElementKind::Copyright
            | ElementKind::Date
            | ElementKind::DoctestBlock
            | ElementKind::Emphasis
            | ElementKind::FieldName
            | ElementKind::Generated
            | ElementKind::Inline
            | ElementKind::Line
            | ElementKind::Literal
            | ElementKind::LiteralBlock
            | ElementKind::Organization
            | ElementKind::Paragraph
            | ElementKind::Problematic
            | ElementKind::Reference
            | ElementKind::Revision
            | ElementKind::Rubric
            | ElementKind::Status
            | ElementKind::Strong
            | ElementKind::Subscript
            | ElementKind::SubstitutionDefinition
            | ElementKind::SubstitutionReference
            | ElementKind::Subtitle
            | ElementKind::Superscript
            | ElementKind::Term
            | ElementKind::Title
            | ElementKind::TitleReference
            | ElementKind::Version => ContentModel::TextOrInline,

            _ => ContentModel::ChildrenOnly,
        }
    }

    pub fn has_category(self, category: ElementCategory) -> bool {
        self.categories().contains(&category)
    }
}
