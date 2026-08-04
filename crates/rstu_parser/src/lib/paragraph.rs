// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef};

use crate::parser_errors::FindElementError;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use crate::token_slice::{find_next_kind, tokens_to_text};

pub(crate) fn try_parse_paragraph(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let paragraph_end = find_next_kind(
        tokens,
        &[
            TK::BlankLine,
            TK::Indent,
            TK::Separator,
            TK::Dedent,
            TK::Field,
        ],
        start_at,
    )
    .expect("Paragraph must end somewhere.");
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    let mut index = start_at;
    while index < paragraph_end {
        let (node, new_index) = match tokens[index].kind {
            kind if kind.is(TC::INLINE_MARKER) => try_parse_inline(&tokens, index)?,
            kind if kind.is(TC::PLAIN) || kind == TK::Punctuation => {
                try_parse_plain(&tokens, index)?
            }
            kind if kind.is(TC::CONTROL) => {
                index += 1;
                continue;
            }
            _ => {
                return Err(FindElementError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", tokens[index].kind),
                });
            }
        };
        index = new_index;
        AstNode::push_child(&paragraph, node);
    }
    Ok((paragraph, index))
}

fn try_parse_inline(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let inline_final = match tokens[start_at].kind {
        TK::Strong => find_next_kind(tokens, &[TK::Strong], start_at + 1)
            .map_err(|_| FindElementError::StrongMissingClosing { start_at: start_at })?,
        _ => {
            return Err(FindElementError::UnexpectedToken {
                expected: "Inline".to_owned(),
                found: format!("{:?}", tokens[start_at].kind),
            });
        }
    };
    let strong = AstNode::new_ref(NodeClass::InlineMarkup);
    AstNode::with_attr(&strong, "markup", "strong");
    AstNode::with_text(&strong, tokens_to_text(&tokens[start_at + 1..inline_final]));
    Ok((strong, inline_final + 1))
}

fn try_parse_plain(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let mut plain_tokens = start_at;
    let mut text = String::new();
    while tokens[plain_tokens].is(&[TK::Word, TK::Spaces, TK::Punctuation, TK::NewLine]) {
        text.push_str(&tokens[plain_tokens].lexeme);
        plain_tokens += 1;
    }

    let sentence = AstNode::new_ref(NodeClass::PlainText);
    AstNode::with_text(&sentence, text);
    Ok((sentence, plain_tokens))
}
