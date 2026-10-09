// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum NodeClass {
    BlankLine,
    Block,
    BlockQuote,
    BulletList,
    BulletListItem,
    Classifier,
    Comment,
    Definition,
    DefinitionList,
    DefinitionListItem,
    Directive,
    Document,
    EnumeratedList,
    EnumeratedListItem,
    FieldList,
    FieldListItem,
    InlineMarkup,
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
    Term,
    Title,
}
