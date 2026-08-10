// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_indented_block_hanging(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {

    // look-adhead line_end_index
    // if there is an indent, go ahead until the "paragraph end" (dedent/indent/blankline)
    //     store the indent in the indented block
    // if no indent, parse until eol

    // Implementation options:
    // index mapping
    // reorder tokens
    // pass skip indices list
}
