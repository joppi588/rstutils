// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::parsers::hyperlink_reference::{
    normalize_reference_name, parse_reference_expression, parse_reference_plain,
    parse_reference_token,
};
use crate::token::TokenCategory as TC;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use crate::with_hanging_indent;

pub(crate) fn parse_inline_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind == TK::InlineInternalTargetStart);

    let position = stream.position_at_cursor();
    stream.consume(); // inline target marker

    let text = stream.consume_text_until(&[&[TK::BackquoteEnd], TC::PARAGRAPH_END].concat());
    if stream.token_at_cursor().kind != TK::BackquoteEnd {
        return Err(ParserError::TargetMissingClosing {
            markup: "inline".to_owned(),
            position,
        });
    }
    stream.consume();
    let target = AstNode::new_ref(NodeClass::Target)
        .with_attr("markup", "inline")
        .with_attr("label", text);
    Ok(target)
}

pub(crate) fn parse_hyperlink_target(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let marker = stream.consume();
    debug_assert!(matches!(
        marker.kind,
        TK::HyperlinkTarget
            | TK::HyperlinkTargetAnonymous
            | TK::HyperlinkTargetAnonymousShort
            | TK::HyperlinkTargetQuote
    ));

    let target = AstNode::new_ref(NodeClass::Target);
    match marker.kind {
        TK::HyperlinkTargetAnonymous => {
            target.with_attr("markup", "standalone_anonymous");
        }
        TK::HyperlinkTargetAnonymousShort => {
            target.with_attr("markup", "standalone_anonymous_short");
        }
        TK::HyperlinkTarget => {
            let refname = marker
                .lexeme
                .strip_prefix(".. _")
                .and_then(|refname| refname.strip_suffix(':'))
                .expect("There is a refname.");
            target
                .with_attr("label", normalize_reference_name(&refname))
                .with_attr("label_raw", refname);
        }
        TK::HyperlinkTargetQuote => {
            let position = stream.position_at_cursor();
            stream.consume(); // HyperlinkTargetQuote token

            // TODO: wrap this in a with hanging indent, e.g. have a shared function with inline target above

            let refname =
                stream.consume_text_until(&[&[TK::BackquoteEnd], TC::PARAGRAPH_END].concat());
            if stream.consume().kind != TK::BackquoteEnd {
                return Err(ParserError::TargetMissingClosing {
                    markup: "reference".to_owned(),
                    position,
                });
            }
            target
                .with_attr("label", normalize_reference_name(&refname))
                .with_attr("rlabel_raw", refname);
        }
        _ => unreachable!(),
    }
    if stream.token_at_cursor().kind == TK::Spaces {
        target.push_spaces(stream.consume().len());
    }

    // internal: empty
    // external: uri or e-mail
    // indirect: simple or quoted internal ref
    // external and indirect might be multi-line
    match (
        stream.token_at_cursor().kind,
        stream.token_at_nextline().kind,
    ) {
        (TK::BackquoteStart, TK::Indent) => {
            target.push_child(with_hanging_indent(stream, parse_reference_expression)?);
            stream.consume_newline();
        }
        (_, TK::Indent) => {
            target.push_child(with_hanging_indent(stream, parse_reference_plain)?);
        }
        (TK::NewLine, kind) if kind.is(TC::PARAGRAPH_END) => {
            target.with_attr("type", "internal");
            stream.consume();
        }
        (kind1, kind2) if kind1.is(TC::INLINE_REFERENCE) && kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_token(stream)?);
            stream.consume_newline();
        }
        (TK::BackquoteStart, kind2) if kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_expression(stream)?);
            stream.consume_newline();
        }
        (_, kind2) if kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_plain(stream)?);
        }
        _ => {
            return Err(ParserError::TargetHyperlinkMalformed {
                position: stream.position_at_cursor(),
            });
        }
    }
    Ok(target)
}
