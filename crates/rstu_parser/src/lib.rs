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
use crate::token::{TokenCategory as TC, TokenKind as TK};
use parser_errors::{ParserError, EXPECT_NEWLINE};
use token_stream::{tokens_to_text, TokenStream};

// static DEDENT_GRACE: usize = 1;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, ParserError> {
    let mut stream = TokenStream::new(tokenize(input));
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut current_parent = doc.clone();

    loop {
        match (stream.kind_at_cursor(), stream.kind_at_nextline()) {
            (TK::Separator, TK::Indent | TK::Word) | (TK::Word, TK::Separator) => {
                let section = match_section_header(&mut stream)?;
                current_parent.push_section_ref(section.clone());
                current_parent = section;
            }

            (TK::DoubleDot, _) => {
                let directive = parse_directive_like(&mut stream)?;
                current_parent.push_child(directive);
            }

            (TK::BulletListMarker, _) => {
                let bullet_list = parse_bullet_list(&mut stream)?;
                current_parent.push_child(bullet_list);
            }

            (TK::Field, _) => {
                let field_list = parse_field_list(&mut stream)?;
                current_parent.push_child(field_list);
            }

            (TK::BlankLine, _) => {
                let token = stream.consume();
                current_parent.push_blank_lines(token.lexeme.len());
            }
            // TODO: Do not simply ignore these
            (TK::Indent, _) | (TK::Dedent, _) => {
                stream.consume();
            }

            (kind, _)
                if kind.is(TC::INLINE_MARKER)
                    || kind.is(TC::INLINE_TOKEN)
                    || kind.is(TC::PLAIN) =>
            {
                let paragraph = parse_paragraph(&mut stream, None)?;
                current_parent.push_child(paragraph);
            }
            (TK::EoF, _) => {
                break;
            }
            _ => panic!(
                "Unexpected token combination ({:?},{:?})",
                stream.kind_at_cursor(),
                stream.kind_at_nextline()
            ),
        };
    }

    Ok(doc)
}

pub fn match_section_header(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
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
    let closing_len = closing_token.lexeme.len();
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

/// Parse directives, comments, citations, substitutions
fn parse_directive_like(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let index = stream
        .find_next_kind(&[
            TK::NewLine,
            TK::DoubleColon,
            TK::FootnoteReference,
            TK::HyperlinkReferenceEnd,
            TK::SubstitutionReference,
        ])
        .expect(EXPECT_NEWLINE);
    let directive = match stream.tokens()[index].kind {
        TK::NewLine => parse_comment(stream, index)?,
        TK::DoubleColon => parse_directive(stream, index)?,
        _ => panic!("Not implemented directive-like structure."),
    };
    Ok(directive)
}
