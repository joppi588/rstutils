// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::TokenKind as TK;
use crate::token_stream::{tokens_to_text, TokenStream};
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub fn parse_section_header(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let section = AstNode::new_ref(NodeClass::Section);

    let start_at = stream.cursor();
    let mut opening_style: Option<String> = None;

    if stream.kind_at_cursor() == TK::Separator {
        let opening_token = stream.consume();
        opening_style = Some(opening_token.lexeme[..1].to_string());
        section.with_attr("marker_len_opening", opening_token.len());
        stream.consume(); //Newline
    }
    let title = AstNode::new_ref(NodeClass::Title);
    let title_end = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE);
    title.with_attr(
        "text",
        tokens_to_text(&stream.tokens()[stream.cursor()..title_end]),
    );
    section.push_child(title);

    let closing_index = title_end + 1;
    if stream.kind_at(closing_index) != TK::Separator {
        return Err(ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    stream.set_cursor(closing_index);
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

    stream.consume(); //Newline
    Ok(section)
}
