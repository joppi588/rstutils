// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;
pub mod token_slice;

use rstu_ast::{AstNode, ElementKind, NodeRef};

use crate::token::TokenKind;
use crate::token_slice::TokenSlice;

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
}

pub fn parse(input: &str) -> Result<NodeRef, FindElementError> {
    let tokens = TokenSlice::from_string(input);
    let doc = AstNode::new_ref(ElementKind::Document);
    let mut index: usize = 0;
    let mut current_node = doc.clone();

    while index < tokens.len() {
        match tokens
            .get(index)
            .expect("loop index is always a valid token index")
            .kind
        {
            TokenKind::SectionTitlePrefix => {
                let (section, next_start) = try_match_section_header_prefix(&tokens, index)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = next_start;
            }
            TokenKind::SectionTitleSuffix => {
                let (section, next_start) = try_match_section_header_suffix(&tokens, index)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = next_start;
            }

            _ => index += 1,
        };
    }

    Ok(doc)
}

pub fn try_match_section_header_prefix(
    tokens: &TokenSlice,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    if start_at >= tokens.len() {
        return Err(FindElementError::StartAtOutOfBounds {
            start_at,
            token_count: tokens.len(),
        });
    }

    let next_line_end = find_next_newline(tokens, start_at + 2).ok_or(
        FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        },
    )?;

    let closing_index = next_line_end + 1;
    if (closing_index >= tokens.len())
        || (tokens
            .get(closing_index)
            .expect("index is checked against bounds")
            .kind
            != TokenKind::SectionTitleSuffix)
    {
        return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }

    let opening_style = tokens
        .get(start_at)
        .expect("start index is validated")
        .lexeme
        .clone(); // TODO: single char + opening/closing length
    let closing_style = tokens
        .get(closing_index)
        .expect("closing index is validated")
        .lexeme
        .clone();
    if opening_style != closing_style {
        return Err(FindElementError::SectionTitleUnbalancedStyle {
            opening_index: start_at,
            opening_style,
            closing_style,
        });
    }

    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "section_marker", closing_style.clone());

    let title = AstNode::new_ref(ElementKind::Title);
    let title_tokens = tokens
        .slice(start_at + 1..closing_index)
        .expect("title slice bounds are already validated");
    AstNode::with_text(&title, title_tokens.to_text());
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, closing_index + 1))
}

pub fn try_match_section_header_suffix(
    tokens: &TokenSlice,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    if start_at >= tokens.len() {
        return Err(FindElementError::StartAtOutOfBounds {
            start_at,
            token_count: tokens.len(),
        });
    }

    let previous_line_start = move_back_one_line(tokens, start_at).unwrap_or(0);
    let closing_style = tokens
        .get(start_at)
        .expect("start index is validated")
        .lexeme
        .clone();

    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "section_marker", closing_style.clone());

    let title = AstNode::new_ref(ElementKind::Title);
    let title_tokens = tokens
        .slice(previous_line_start..start_at)
        .expect("title slice bounds are already validated");
    AstNode::with_text(&title, title_tokens.to_text());
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, start_at + 1))
}

fn find_next_newline(tokens: &TokenSlice, start_at: usize) -> Option<usize> {
    tokens
        .as_slice()
        .iter()
        .enumerate()
        .skip(start_at)
        .find_map(|(index, token)| (token.kind == TokenKind::NewLine).then_some(index))
}

fn move_back_one_line(tokens: &TokenSlice, index: usize) -> Option<usize> {
    // Move to the first token of the line ending before index
    let mut cursor = index.checked_sub(2)?;
    let token_values = tokens.as_slice();

    while !matches!(
        token_values[cursor].kind,
        TokenKind::NewLine | TokenKind::BlankLine
    ) {
        cursor = cursor.checked_sub(1)?;
    }

    Some(cursor + 1)
}
