// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub fn parse_section_header(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let section = AstNode::new_ref(NodeClass::Section);

    let start_at = stream.cursor();
    let mut opening_style: Option<String> = None;

    if stream.token_at_cursor().kind == TK::Separator {
        let opening_token = stream.consume();
        opening_style = Some(opening_token.lexeme[..1].to_string());
        section.with_attr("marker_len_opening", opening_token.len());
        stream.consume_newline();
    }
    let title = AstNode::new_ref(NodeClass::Title);
    title.with_attr(
        "text",
        stream
            .consume_text_until(&[TK::NewLine])
            .expect(EXPECT_NEWLINE), // TODO: Do not only check for NewLine
    );
    section.push_child(title);

    if stream.token_at_cursor().kind != TK::Separator {
        return Err(ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    let closing_token = stream.consume();
    let closing_style: String = closing_token.lexeme[..1].to_string();
    let closing_len = closing_token.len();

    if let Some(opening_style) = opening_style.filter(|style| style != &closing_style) {
        return Err(ParserError::SectionTitleUnbalancedStyle {
            opening_index: start_at,
            opening_style,
            closing_style,
        });
    }

    section
        .with_attr("section_marker", closing_style)
        .with_attr("marker_len", closing_len);

    stream.consume_newline();
    Ok(section)
}
