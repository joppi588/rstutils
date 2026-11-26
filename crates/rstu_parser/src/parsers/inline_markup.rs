// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use crate::parser_errors::ParserError;
use crate::parsers::hyperlink_reference::{parse_reference_expression, parse_reference_token};
use crate::parsers::targets::parse_inline_target;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn parse_inline_markup(
    stream: &mut TokenStream,
) -> Result<Option<NodeRef>, ParserError> {
    let node = match stream.token_at_cursor().kind {
        TK::RolePrefix => parse_prefix_role(stream)?,
        TK::StrongStart => parse_strong(stream)?,
        TK::EmphasisStart => parse_emphasis(stream)?,
        TK::InlineLiteralStart => parse_inline_literal(stream)?,
        kind if kind.is(TC::INLINE_REFERENCE) => parse_reference_token(stream)?,
        TK::InlineInternalTargetStart => parse_inline_target(stream)?,
        TK::BackquoteStart => parse_interpreted_or_ref(stream)?,
        _ => return Ok(None),
    };
    Ok(Some(node))
}

pub(crate) fn parse_interpreted_or_ref(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::BackquoteStart);

    let position = stream.position_at_cursor();

    let end_kind_index = stream
        .find_next_kind(&[TC::BACKQUOTE, TC::PARAGRAPH_END].concat())
        .expect("At least find EoF");
    let end_kind = stream.token_at(end_kind_index).kind;

    let node = match end_kind {
        TK::HyperlinkReferenceAnonymousEnd | TK::HyperlinkReferenceEnd => {
            parse_reference_expression(stream)?.with_attr("markup", "inline")
        }
        TK::BackquoteEnd => {
            stream.consume(); // Starting Backquote
            let text = stream.consume_text_until(&[TK::BackquoteEnd]);
            stream.consume(); // Closing Backquote
            AstNode::new_ref(NodeClass::InterpretedText).with_attr("text", text)
        }
        TK::RolePostfix => {
            stream.consume();
            let text = stream.consume_text_until(&[TK::RolePostfix]);
            let role_lexeme = &stream.consume().lexeme;
            AstNode::new_ref(NodeClass::InterpretedText)
                .with_attr("text", text)
                .with_attr("role", &role_lexeme[2..role_lexeme.len() - 1])
                .with_attr("pos", "postfix".to_owned())
        }
        _ => {
            return Err(ParserError::InlineMissingClosing {
                markup: "interpreted text or reference".to_owned(),
                position,
            });
        }
    };
    Ok(node)
}

pub(crate) fn parse_prefix_role(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::RolePrefix);

    let position = stream.position_at_cursor();
    let token = stream.consume();
    let text = stream.consume_text_until(&[TC::BACKQUOTE, TC::PARAGRAPH_END].concat());
    let end_kind = stream.consume().kind;

    match end_kind {
        TK::RolePostfix => {
            return Err(ParserError::RolePrefixAndPostfix { position });
        }
        TK::HyperlinkReferenceEnd | TK::HyperlinkReferenceAnonymousEnd => {
            return Err(ParserError::RoleReferenceMismatch { position });
        }
        TK::BackquoteEnd => {}
        _ => {
            return Err(ParserError::InlineMissingClosing {
                markup: format!("{:?}", token.kind),
                position,
            });
        }
    }

    let role = &token.lexeme;
    Ok(AstNode::new_ref(NodeClass::InterpretedText)
        .with_attr("role", &role[1..role.len() - 2])
        .with_attr("pos", "prefix".to_owned())
        .with_attr("text", text))
}

// TODO: Error on consecutive Start Tokens.
pub(crate) fn parse_strong(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::StrongStart);

    let position = stream.position_at_cursor();
    stream.consume();
    let text = stream.consume_text_until(&[&[TK::StrongEnd], TC::PARAGRAPH_END].concat());
    if stream.consume().kind != TK::StrongEnd {
        return Err(ParserError::InlineMissingClosing {
            markup: format!("{:?}", TK::StrongStart),
            position,
        });
    }
    Ok(AstNode::new_ref(NodeClass::Strong).with_attr("text", text))
}

pub(crate) fn parse_emphasis(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::EmphasisStart);

    let position = stream.position_at_cursor();
    stream.consume();
    let text = stream.consume_text_until(&[&[TK::EmphasisEnd], TC::PARAGRAPH_END].concat());
    if stream.consume().kind != TK::EmphasisEnd {
        return Err(ParserError::InlineMissingClosing {
            markup: format!("{:?}", TK::EmphasisStart),
            position,
        });
    }
    Ok(AstNode::new_ref(NodeClass::Emphasis).with_attr("text", text))
}

pub(crate) fn parse_inline_literal(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::InlineLiteralStart);

    let position = stream.position_at_cursor();
    stream.consume();
    let text = stream.consume_text_until(&[&[TK::InlineLiteralEnd], TC::PARAGRAPH_END].concat());
    if stream.consume().kind != TK::InlineLiteralEnd {
        return Err(ParserError::InlineMissingClosing {
            markup: format!("{:?}", TK::InlineLiteralStart),
            position,
        });
    }
    Ok(AstNode::new_ref(NodeClass::Literal).with_attr("text", text))
}
