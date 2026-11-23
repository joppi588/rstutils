// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::normalize_name;
use crate::parser_errors::ParserError;
use crate::parsers::hyperlink_reference::{
    parse_reference_expression, parse_reference_plain, parse_reference_token,
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
                .with_attr("label", normalize_name(&refname))
                .with_attr("raw", refname);
        }
        TK::HyperlinkTargetQuote => {
            let position = stream.position_at_cursor();
            stream.consume(); // HyperlinkTargetQuote token

            // TODO: wrap this in a with hanging indent, e.g. have a shared function with inline target above

            let refname = stream.consume_text_until(&[TK::BackquoteEnd, TK::EoF]);
            if stream.consume().kind != TK::BackquoteEnd {
                return Err(ParserError::TargetMissingClosing {
                    markup: "reference".to_owned(),
                    position,
                });
            }
            let colon_expected = stream.consume().lexeme;
            if colon_expected != ":" {
                return Err(ParserError::UnexpectedToken {
                    expected: ":".to_owned(),
                    found: colon_expected,
                    position: stream.position_at_cursor(),
                });
            }
            target
                .with_attr("label", normalize_name(&refname))
                .with_attr("raw", refname);
        }
        _ => unreachable!(),
    }
    if stream.token_at_cursor().kind == TK::Spaces {
        target.push_spaces(stream.consume().len());
    }

    // 1. internal: empty
    // 2. external: uri or e-mail
    //    a: single line, b: multi-line
    // 3. indirect:
    //    a: simple
    //    b: quoted internal ref, single-line
    //    c: quoted internal ref, multi-line
    //    TODO: d: quoted internal ref, starting on next line -> not implemented yet
    match (
        stream.token_at_cursor().kind,
        stream.token_at_nextline().kind,
    ) {
        (TK::BackquoteStart, TK::Indent) => {
            target.push_child(with_hanging_indent(stream, parse_reference_expression)?);
            // 3c
        }
        (_, TK::Indent) => {
            target.push_child(with_hanging_indent(stream, parse_reference_plain)?);
            // 2b
        }
        (TK::NewLine, kind) if kind.is(TC::PARAGRAPH_END) => {
            stream.consume(); // 1
        }
        (kind1, kind2) if kind1.is(TC::INLINE_REFERENCE) && kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_token(stream)?); //3a
            stream.consume_newline();
        }
        (TK::BackquoteStart, kind2) if kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_expression(stream)?); //3b
            stream.consume_newline();
        }
        (_, kind2) if kind2.is(TC::PARAGRAPH_END) => {
            target.push_child(parse_reference_plain(stream)?); //2a
        }
        (_, kind2) if !kind2.is(TC::PARAGRAPH_END) => {
            return Err(ParserError::UnexpectedToken {
                expected: "Paragraph end".to_owned(),
                found: format!("{kind2:?}"),
                position: stream.position_at(stream.find_end_of_line() + 1),
            });
        }
        _ => {
            return Err(ParserError::TargetHyperlinkMalformed {
                position: stream.position_at_cursor(),
            });
        }
    }
    Ok(target)
}
