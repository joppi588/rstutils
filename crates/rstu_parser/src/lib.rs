// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
mod parsers;
use parsers::comments::parse_comment;
use parsers::directives::parse_directive;
use parsers::list::{parse_bullet_list, parse_field_list};
use parsers::paragraph::parse_paragraph;

pub mod parser_errors;
pub mod token;
pub mod token_stream;

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::lexer::tokenize;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use parser_errors::{ParserError, EXPECT_NEWLINE};
use token_stream::{find_next_kind, tokens_to_text, TokenSliceExt};

// static DEDENT_GRACE: usize = 1;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, ParserError> {
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut index: usize = 0;
    let mut current_parent = doc.clone();

    loop {
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine], index).unwrap_or(tokens.len()); // TODO: Integrate this in token stream.
        match (tokens.kind_at(index), tokens.kind_at(index_line_end + 1)) {
            (TK::Separator, TK::Indent | TK::Word) | (TK::Word, TK::Separator) => {
                let (section, next_start) = match_section_header(&tokens, index)?;
                current_parent.push_section_ref(section.clone());
                current_parent = section;
                index = next_start;
            }

            (TK::DoubleDot, _) => {
                let (directive, next_start) = parse_directive_like(&tokens, index)?;
                current_parent.push_child(directive);
                index = next_start;
            }

            (TK::BulletListMarker, _) => {
                let (bullet_list, next_start) = parse_bullet_list(&tokens, index)?;
                current_parent.push_child(bullet_list);
                index = next_start;
            }

            (TK::Field, _) => {
                let (field_list, next_start) = parse_field_list(&tokens, index)?;
                current_parent.push_child(field_list);
                index = next_start;
            }

            (TK::BlankLine, _) => {
                current_parent.push_blank_lines(tokens[index].lexeme.len());
                index += 1;
            }
            // TODO: Do not simply ignore these
            (TK::Indent, _) | (TK::Dedent, _) => index += 1,

            (kind, _)
                if kind.is(TC::INLINE_MARKER)
                    || kind.is(TC::INLINE_TOKEN)
                    || kind.is(TC::PLAIN) =>
            {
                let (paragraph, next_start) = parse_paragraph(&tokens, index, None)?;
                current_parent.push_child(paragraph);
                index = next_start;
            }
            (TK::EoF, _) => {
                break;
            }
            _ => panic!(
                "Unexpected token combination ({:?},{:?})",
                tokens.kind_at(index),
                tokens.kind_at(index_line_end + 1)
            ),
        };
    }

    Ok(doc)
}

pub fn match_section_header(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let has_overline = tokens.kind_at(start_at) == TK::Separator;

    let title_start = start_at + 2 * usize::from(has_overline);
    let title_end = find_next_kind(tokens, &[TK::NewLine], title_start).map_err(|_| {
        ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        }
    })?;

    let closing_index = title_end + 1;
    if tokens.kind_at(closing_index) != TK::Separator {
        return Err(ParserError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    let closing_token = &tokens[closing_index];
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
    title.with_attr("text", tokens_to_text(&tokens[title_start..title_end]));
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
    )
    .expect(EXPECT_NEWLINE);
    let (directive, new_index) = match &tokens[index].kind {
        TK::NewLine => parse_comment(tokens, start_at, index)?,
        TK::DoubleColon => parse_directive(tokens, start_at, index)?,
        _ => panic!("Not implemented directive-like structure."),
    };
    Ok((directive, new_index))
}
