// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::{tokens_to_text, TokenStream};

pub(crate) fn parse_paragraph(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let paragraph_end = stream
        .find_next_kind(&[
            TK::BlankLine,
            TK::Separator,
            TK::Indent,
            TK::Dedent,
            TK::EoF,
        ])
        .expect("Paragraph must end somewhere.");
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    while stream.cursor() < paragraph_end {
        let kind = stream.kind_at_cursor();
        let node = match kind {
            kind if kind.is(TC::INLINE_MARKER) => parse_inline(stream)?,
            kind if kind.is(TC::INLINE_TOKEN) => parse_inline_token(stream)?,
            //TODO: Concatenate TC::PLAIN and tokens to a new list
            kind if kind.is(TC::PLAIN) || kind == TK::BulletListMarker || kind == TK::NewLine => {
                parse_plain(stream, paragraph_end)?
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", kind),
                    index: stream.cursor(),
                });
            }
        };
        paragraph.push_child(node);
    }
    Ok(paragraph)
}

pub(crate) fn parse_inline_token(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let at = stream.cursor();
    let node = AstNode::new_ref(NodeClass::Reference);
    let token = stream.consume();
    let kind = token.kind;
    let lexeme = &token.lexeme;
    match kind {
        TK::FootnoteReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "footnote");
        }
        TK::SubstitutionReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 1])
                .with_attr("type", "sub");
        }
        TK::SimpleHyperlinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 1])
                .with_attr("type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 2])
                .with_attr("type", "simple_anonymous_ref");
        }

        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Reference token".to_owned(),
                found: format!("{:?}", kind),
                index: at,
            });
        }
    };
    Ok(node)
}

pub(crate) fn parse_inline(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let start_at = stream.cursor();
    let kind = stream.kind_at_cursor();
    let (markup, end_kind_candidates): (&str, &[TK]) = match kind {
        TK::StrongStart => ("strong", &[TK::StrongEnd]),
        TK::EmphasisStart => ("emphasis", &[TK::EmphasisEnd]),
        TK::InlineLiteralStart => ("inline_literal", &[TK::InlineLiteralEnd]),
        TK::InlineInternalTargetStart => ("inline_internal_target", &[TK::BackquoteEnd]),
        TK::BackquoteStart => (
            "interpreted_or_hyperlink",
            &[TK::BackquoteEnd, TK::HyperlinkReferenceEnd],
        ),
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Inline start token".to_owned(),
                found: format!("{:?}", kind),
                index: start_at,
            });
        }
    };

    let inline_final = stream
        .find_next_kind_from(end_kind_candidates, start_at + 1)
        .map_err(|_| ParserError::InlineMissingClosing {
            markup: markup.to_owned(),
            start_at,
        })?;

    let effective_markup = match (kind, stream.tokens()[inline_final].kind) {
        (TK::BackquoteStart, TK::HyperlinkReferenceEnd) => "hyperlink_reference",
        (TK::BackquoteStart, TK::BackquoteEnd) => "interpreted_text",
        _ => markup,
    };

    let inline = AstNode::new_ref(NodeClass::InlineMarkup);
    inline.with_attr("markup", effective_markup).with_attr(
        "text",
        tokens_to_text(&stream.tokens()[start_at + 1..inline_final]),
    );
    stream.set_cursor(inline_final + 1);
    Ok(inline)
}

fn parse_plain(stream: &mut TokenStream, stop_before: usize) -> Result<NodeRef, ParserError> {
    let mut text = String::new();
    while stream.cursor() < stop_before
        && stream.kind_at_cursor().is(&[
            TK::Word,
            TK::Spaces,
            TK::Punctuation,
            TK::BulletListMarker,
            TK::NewLine,
        ])
    {
        // TODO: We had this in the paragraph parser already, DRY (PLAIN || Bulletlistmarker)
        text.push_str(&stream.consume().lexeme);
    }

    let sentence = AstNode::new_ref(NodeClass::PlainText);
    sentence.with_attr("text", text);
    Ok(sentence)
}
