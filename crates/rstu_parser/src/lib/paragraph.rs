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
            kind if kind.is(TC::INLINE_TOKEN) => try_parse_inline_ref(&tokens, index)?,
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

pub(crate) fn try_parse_inline_ref(
    tokens: &[Token],
    at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let node = AstNode::new_ref(NodeClass::Reference);
    let kind = tokens[at].kind;
    let lexeme = &tokens[at].lexeme;
    match kind {
        TK::FootnoteReference => {
            AstNode::with_text(&node, &lexeme[1..lexeme.len() - 2]);
            AstNode::with_attr(&node, "type", "footnote");
        }
        TK::SubstitutionReference => {
            AstNode::with_text(&node, &lexeme[1..lexeme.len() - 1]);
            AstNode::with_attr(&node, "type", "sub");
        }
        TK::SimpleHyperlinkReference => {
            AstNode::with_text(&node, &lexeme[0..lexeme.len() - 1]);
            AstNode::with_attr(&node, "type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            AstNode::with_text(&node, &lexeme[0..lexeme.len() - 2]);
            AstNode::with_attr(&node, "type", "simple_anonymous_ref");
        }

        _ => {
            return Err(FindElementError::UnexpectedToken {
                expected: "Reference token".to_owned(),
                found: format!("{:?}", kind),
            });
        }
    };
    Ok((node, at + 1))
}

pub(crate) fn try_parse_inline(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let (markup, end_kind_candidates): (&str, &[TK]) = match tokens[start_at].kind {
        TK::StrongStart => ("strong", &[TK::StrongEnd]),
        TK::EmphasisStart => ("emphasis", &[TK::EmphasisEnd]),
        TK::InlineLiteralStart => ("inline_literal", &[TK::InlineLiteralEnd]),
        TK::InlineInternalTargetStart => ("inline_internal_target", &[TK::BackquoteEnd]),
        TK::BackquoteStart => (
            "interpreted_or_hyperlink",
            &[TK::BackquoteEnd, TK::HyperlinkReferenceEnd],
        ),
        _ => {
            return Err(FindElementError::UnexpectedToken {
                expected: "Inline start token".to_owned(),
                found: format!("{:?}", tokens[start_at].kind),
            });
        }
    };

    let inline_final = find_next_kind(tokens, end_kind_candidates, start_at + 1).map_err(|_| {
        FindElementError::InlineMissingClosing {
            markup: markup.to_owned(),
            start_at,
        }
    })?;

    let effective_markup = if tokens[start_at].kind == TK::BackquoteStart {
        if tokens[inline_final].kind == TK::HyperlinkReferenceEnd {
            "hyperlink_reference"
        } else {
            "interpreted_text"
        }
    } else {
        markup
    };

    let inline = AstNode::new_ref(NodeClass::InlineMarkup);
    AstNode::with_attr(&inline, "markup", effective_markup);
    AstNode::with_text(&inline, tokens_to_text(&tokens[start_at + 1..inline_final]));
    Ok((inline, inline_final + 1))
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
