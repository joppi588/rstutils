// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

mod comments;
mod directives;
pub mod lexer;
#[path = "lib/list.rs"]
mod list;
#[path = "lib/paragraph.rs"]
mod paragraph;

pub mod parser_errors;
pub mod token;
pub mod token_slice;

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::lexer::tokenize;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use parser_errors::{ParserError, EXPECT_NEWLINE};
use token_slice::{find_next_kind, tokens_to_text};

// static DEDENT_GRACE: usize = 1;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, ParserError> {
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut index: usize = 0;
    let mut current_node = doc.clone();

    while index < tokens.len() - 2 {
        // final two tokens are always Newline+Blankline
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine], index, None)
            .expect("Token stream ends with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (token1, token2)
                if (token1, token2) == (TK::Separator, TK::Indent)
                    || (token1, token2) == (TK::Separator, TK::Word)
                    || (token1, token2) == (TK::Word, TK::Separator) =>
            {
                let (section, next_start) =
                    match_section_header(&tokens, index, token1.is(&[TK::Separator]))?;
                current_node.push_section_ref(section.clone());
                current_node = section;
                index = next_start;
            }

            (TK::DoubleDot, _) => {
                let (directive, next_start) = parse_directive_like(&tokens, index)?;
                current_node.push_body_element(directive.clone());
                current_node = directive;
                index = next_start;
            }

            (TK::BulletListMarker, _) => {
                let (bullet_list, next_start) = list::parse_bullet_list(&tokens, index)?;
                current_node.push_body_element(bullet_list);
                index = next_start;
            }

            (TK::Field, _) => {
                let (field_list, next_start) = list::parse_field_list(&tokens, index)?;
                current_node.push_body_element(field_list);
                index = next_start;
            }

            (TK::NewLine, TK::BlankLine)
            | (TK::BlankLine, _)
            | (TK::Indent, _)
            | (TK::Dedent, _) => index += 1,

            (kind, _)
                if kind.is(TC::INLINE_MARKER)
                    || kind.is(TC::INLINE_TOKEN)
                    || kind.is(TC::PLAIN) =>
            {
                let (paragraph, next_start) =
                    paragraph::parse_paragraph(&tokens, index, None, None)?;
                current_node.push_child(paragraph.clone());
                index = next_start;
            }

            _ => panic!(
                "Unexpected token combination ({:?},{:?})",
                tokens[index].kind,
                tokens[index_line_end + 1].kind
            ),
        };
    }

    Ok(doc)
}

pub fn match_section_header(
    tokens: &[Token],
    start_at: usize,
    has_overline: bool,
) -> Result<(NodeRef, usize), ParserError> {
    let title_start = start_at + 2 * usize::from(has_overline);
    let title_end = find_next_kind(tokens, &[TK::NewLine], title_start, None).map_err(|_| {
        ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        }
    })?;

    let closing_index = title_end + 1;
    let closing_token = &tokens[closing_index];
    if (closing_index >= tokens.len()) || (closing_token.kind != TK::Separator) {
        return Err(ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    let closing_style: String = closing_token.lexeme[..1].to_string();
    let closing_len = closing_token.lexeme.len();

    let opening_len = if has_overline {
        let opening_token = &tokens[start_at];
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
    title.with_text(tokens_to_text(&tokens[title_start..title_end]));
    section.push_child(title);

    Ok((section, closing_index + 2))
}

/// Parse directives, comments, citations, substitutions
fn parse_directive_like(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let index = find_next_kind(
        tokens,
        &[
            TK::NewLine,
            TK::DoubleColon,
            TK::FootnoteReference,
            TK::HyperlinkReferenceEnd,
            TK::SubstitutionReference,
        ],
        start_at,
        None,
    )
    .expect(EXPECT_NEWLINE);
    let (directive, new_index) = match &tokens[index].kind {
        TK::NewLine => comments::parse_comment(tokens, start_at, index)?,
        TK::DoubleColon => directives::parse_directive(tokens, start_at, index)?,
        _ => panic!("Not implemented directive-like structure."),
    };
    Ok((directive, new_index))
}
