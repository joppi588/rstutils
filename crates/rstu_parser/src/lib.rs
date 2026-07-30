// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;
pub mod token_slice;

use rstu_ast::{AstNode, ElementKind, NodeRef};

use crate::lexer::tokenize;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK, TokenKindIs};
use token_slice::{find_next_kind, tokens_to_text, ScanDirection};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindElementError {
    StartAtOutOfBounds {
        start_at: usize,
        token_count: usize,
    },
    SectionTitleMissingClosingAfterOpening {
        opening_index: usize,
    },
    SectionTitleUnbalancedStyle {
        opening_index: usize,
        opening_style: String,
        closing_style: String,
    },
    InvalidPlainText {
        start_at: usize,
    },
    UnexpectedToken {
        expected: String,
        found: String,
    },
    StrongMissingClosing {
        start_at: usize,
    },
}

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, FindElementError> {
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(ElementKind::Document);
    let mut index: usize = 0;
    let mut current_node = doc.clone();

    while index < tokens.len() - 2 {
        // final two tokens are always Newline+Blankline
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine], ScanDirection::Forward, index)
            .expect("Token stream shall end with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (TK::Separator, TK::Indent) | (TK::Separator, TK::Word) => {
                let (section, next_start) = try_match_section_header(&tokens, index, true)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = next_start;
            }
            (TK::Word, TK::Separator) => {
                let (section, index_end_header) = try_match_section_header(&tokens, index, false)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = index_end_header + 1;
            }
            (TK::NewLine, TK::BlankLine) | (TK::BlankLine, _) => index += 1,
            (kind, _) if kind.is(TC::INLINE_MARKER) || kind.is(TC::PLAIN) => {
                let (paragraph, next_start) = try_parse_paragraph(&tokens, index)?;
                AstNode::push_child(&current_node, paragraph.clone())
                    .expect("Structural node can have children.");
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

pub fn try_match_section_header(
    tokens: &[Token],
    start_at: usize,
    skip_prefix: bool,
) -> Result<(NodeRef, usize), FindElementError> {
    let title_start = start_at + usize::from(skip_prefix);
    let line_search_start = if skip_prefix { start_at + 2 } else { start_at };
    let line_end = find_next_kind(
        tokens,
        &[TK::NewLine],
        ScanDirection::Forward,
        line_search_start,
    )
    .map_err(
        |_| FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        },
    )?;

    let closing_index = line_end + 1;
    if (closing_index >= tokens.len()) || (tokens[closing_index].kind != TK::Separator) {
        return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }

    let closing_style = tokens[closing_index].lexeme.clone();

    if skip_prefix {
        let opening_style = tokens[start_at].lexeme.clone(); // TODO: single char + opening/closing length
        if tokens[start_at].lexeme != tokens[closing_index].lexeme {
            return Err(FindElementError::SectionTitleUnbalancedStyle {
                opening_index: start_at,
                opening_style,
                closing_style: closing_style.clone(),
            });
        }
    }

    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "section_marker", closing_style);

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title, tokens_to_text(&tokens[title_start..line_end + 1]));
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, closing_index + 1))
}

fn try_parse_paragraph(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let paragraph_end = find_next_kind(
        tokens,
        &[TK::BlankLine, TK::Indent, TK::Separator],
        ScanDirection::Forward,
        start_at,
    )
    .expect("Paragraph must end somewhere.");
    let paragraph = AstNode::new_ref(ElementKind::Paragraph);
    let mut index = start_at;
    while index < paragraph_end {
        let (node, new_index) = match tokens[index].kind {
            kind if kind.is(TC::INLINE_MARKER) => try_parse_inline(tokens, index)?,
            kind if kind.is(TC::PLAIN) => try_parse_plain(tokens, index)?,
            kind if kind.is(TC::CONTROL) => {
                index += 1;
                continue;
            }
            _ => {
                return Err(FindElementError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", tokens[index].kind),
                });
            }
        };
        index = new_index;
        AstNode::push_child(&paragraph, node).expect("Paragraph can have children.");
    }
    Ok((paragraph, paragraph_end + 1))
}

fn try_parse_inline(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let inline_final = match tokens[start_at].kind {
        TK::Strong => find_next_kind(tokens, &[TK::Strong], ScanDirection::Forward, start_at + 1)
            .map_err(|_| FindElementError::StrongMissingClosing { start_at: start_at })?,
        _ => {
            return Err(FindElementError::UnexpectedToken {
                expected: "Inline".to_owned(),
                found: format!("{:?}", tokens[start_at].kind),
            });
        }
    };
    let strong = AstNode::new_ref(ElementKind::Strong);
    AstNode::with_attr(
        &strong,
        "text",
        tokens_to_text(&tokens[start_at + 1..inline_final]),
    );
    Ok((strong, inline_final + 1))
}

fn try_parse_plain(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let plain_tokens = find_next_kind(
        tokens,
        &[TK::Strong, TK::BlankLine, TK::DoubleDot], // TODO implement kinds_except
        ScanDirection::Forward,
        start_at,
    )
    .map_err(|_| FindElementError::InvalidPlainText { start_at: start_at })?;
    let sentence = AstNode::new_ref(ElementKind::PlainText);
    AstNode::with_attr(
        &sentence,
        "text",
        tokens_to_text(&tokens[start_at..plain_tokens]),
    );
    Ok((sentence, plain_tokens))
}
