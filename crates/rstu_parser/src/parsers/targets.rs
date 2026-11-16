// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::{normalize_reference_name, normalize_uri};
use crate::parser_errors::ParserError;
use crate::token::TokenCategory as TC;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;

pub(crate) fn parse_inline_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::InlineInternalTargetStart);

    let position = stream.position_at_cursor();
    stream.consume(); // inline target marker

    let text = stream.consume_text_until(&[&[TK::BackquoteEnd], TC::PARAGRAPH_END].concat());
    if stream.token_at_cursor().kind != TK::BackquoteEnd {
        return Err(ParserError::TargetMissingClosing {
            markup: "target".to_owned(),
            position,
        });
    }
    stream.consume();
    let target = AstNode::new_ref(NodeClass::Target)
        .with_attr("markup", "inline")
        .with_attr("refname", text);
    Ok(target)
}

pub(crate) fn parse_hyperlink_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let marker = stream.consume();
    debug_assert!(matches!(
        marker.kind,
        TK::HyperlinkTarget | TK::HyperlinkTargetAnonymous | TK::HyperlinkTargetAnonymousShort
    ));

    let target = AstNode::new_ref(NodeClass::Target).with_attr("target_type", "explicit");
    if marker.kind == TK::HyperlinkTargetAnonymous {
        target.with_attr("markup", true);
    } else if let Some(label) = marker
        .lexeme
        .strip_prefix(".. _")
        .and_then(|label| label.strip_suffix(':'))
    {
        target.with_attr(
            "label",
            normalize_reference_name(label.trim().trim_matches('`')),
        );
    }

    let mut link_block = String::new();
    while !matches!(stream.token_at_cursor().kind, TK::NewLine | TK::EoF) {
        link_block.push_str(&stream.consume().lexeme);
    }
    if stream.token_at_cursor().kind == TK::NewLine {
        stream.consume_newline();
    }

    if stream.token_at_cursor().kind == TK::Indent {
        let mut indent = stream.consume().len();
        loop {
            match stream.token_at_cursor().kind {
                TK::Indent => indent += stream.consume().len(),
                TK::Dedent => {
                    indent = indent.saturating_sub(stream.consume().len());
                    if indent == 0 {
                        break;
                    }
                }
                TK::NewLine => {
                    stream.consume_newline();
                }
                TK::BlankLine | TK::EoF => break,
                _ => link_block.push_str(&stream.consume().lexeme),
            }
        }
    }

    let link_block = link_block.trim();
    if !link_block.is_empty() {
        if has_indirect_reference(link_block) {
            target.with_attr(
                "refname",
                normalize_reference_name(
                    link_block
                        .strip_suffix('_')
                        .unwrap_or(link_block)
                        .trim()
                        .trim_matches('`'),
                ),
            );
        } else {
            let uri = normalize_uri(link_block);
            let uri = if uri.contains('@') && !uri.contains(':') {
                format!("mailto:{uri}")
            } else {
                uri
            };
            target.with_attr("refuri", uri);
        }
    }
    Ok(target)
}

fn has_indirect_reference(link_block: &str) -> bool {
    let trimmed = link_block.trim_end();
    if !trimmed.ends_with('_') {
        return false;
    }
    let backslashes = trimmed[..trimmed.len() - 1]
        .chars()
        .rev()
        .take_while(|character| *character == '\\')
        .count();
    backslashes % 2 == 0
}
