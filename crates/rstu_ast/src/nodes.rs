// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum NodeClass {
    Comment,
    Directive,
    Document,
    BulletListItem,
    BulletList,
    FieldListItem,
    FieldList,
    IndentedBlock,
    IndentedBlockHanging,
    InlineMarkup,
    Paragraph,
    PlainText,
    Reference,
    Section,
    Strong,
    Title,
}
