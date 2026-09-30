// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::{tokens_to_text, TokenStream};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub fn parse_section_header(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let start_at = stream.cursor();
    let has_overline = stream.kind_at_cursor() == TK::Separator;

    let title_start = start_at + 2 * usize::from(has_overline);
    let title_end = stream
        .find_next_kind_from(&[TK::NewLine], title_start)
        .map_err(|_| ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        })?;

    let closing_index = title_end + 1;
    if stream.kind_at(closing_index) != TK::Separator {
        return Err(ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    let closing_token = &stream.tokens()[closing_index];
    let closing_style: String = closing_token.lexeme[..1].to_string();
    let closing_len = closing_token.len();
    let opening_len = if has_overline {
        let opening_token = &stream.tokens()[start_at];
        let opening_style = opening_token.lexeme[..1].to_string();
        if opening_style != closing_style {
            return Err(ParserError::SectionTitleUnbalancedStyle {
                opening_index: start_at,
                opening_style,
                closing_style,
            });
        }
        opening_style.len()
    } else {
        0
    };

    let section = AstNode::new_ref(NodeClass::Section);
    section
        .with_attr("section_marker", closing_style)
        .with_attr("marker_len", closing_len)
        .with_attr("marker_len_opening", opening_len);

    let title = AstNode::new_ref(NodeClass::Title);
    title.with_attr(
        "text",
        tokens_to_text(&stream.tokens()[title_start..title_end]),
    );
    section.push_child(title);

    stream.set_cursor(closing_index + 2);
    Ok(section)
}
