// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum NodeClass {
    Attribution,
    BlankLine,
    Block,
    BlockQuote,
    BulletList,
    BulletListItem,
    Classifier,
    Comment,
    Citation,
    Definition,
    DefinitionList,
    DefinitionListItem,
    Directive,
    DoctestBlock,
    Document,
    EnumeratedList,
    EnumeratedListItem,
    FieldList,
    FieldListItem,
    Footnote,
    Emphasis,
    InterpretedText,
    Line,
    LineBlock,
    Literal,
    LiteralBlock,
    Option,
    OptionGroup,
    OptionList,
    OptionListItem,
    Paragraph,
    PlainText,
    Reference,
    Section,
    Spaces,
    Strong,
    Target,
    Term,
    Title,
    Transition,
}
