// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use rstu_ast::{AstNode, ElementKind, NodeRef};

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

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
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(ElementKind::Document);
    let mut index: usize = 0;
    let mut section_styles: Vec<String> = Vec::new();
    let mut section_stack: Vec<NodeRef> = Vec::new();

    while index < tokens.len() {
        let parsed = match tokens[index].kind {
            TokenKind::SectionTitlePrefix => Some(try_match_section_header_prefix(&tokens, index)?),
            TokenKind::SectionTitleSuffix => Some(try_match_section_header_suffix(&tokens, index)?),
            _ => None,
        };

        let Some((section_header, next_start)) = parsed else {
            index += 1;
            continue;
        };

        let style = {
            let borrowed = section_header.borrow();
            borrowed
                .attributes
                .get("opening_style")
                .filter(|value| !value.is_empty())
                .cloned()
                .or_else(|| borrowed.attributes.get("closing_style").cloned())
                .unwrap_or_default()
        };

        let level = match section_styles.iter().position(|known| known == &style) {
            Some(existing) => existing,
            None => {
                section_styles.push(style);
                section_styles.len() - 1
            }
        };

        section_stack.truncate(level);
        let parent = if level == 0 {
            doc.clone()
        } else {
            section_stack[level - 1].clone()
        };

        AstNode::push_child(&parent, section_header.clone())
            .expect("section placement should produce a valid AST");
        section_stack.push(section_header);
        index = next_start;
    }

    Ok(doc)
}

pub fn try_match_section_header_prefix(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let next_line_end = find_next_newline(tokens, start_at + 2).ok_or(
        FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        },
    )?;

    let closing_index = next_line_end + 1;
    if (closing_index >= tokens.len())
        || (tokens[closing_index].kind != TokenKind::SectionTitleSuffix)
    {
        return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }

    let opening_style = tokens[start_at].lexeme.clone(); // TODO: single char + opening/closing length
    let closing_style = tokens[closing_index].lexeme.clone();
    if tokens[start_at].lexeme != tokens[closing_index].lexeme {
        return Err(FindElementError::SectionTitleUnbalancedStyle {
            opening_index: start_at,
            opening_style: opening_style,
            closing_style: closing_style,
        });
    }
    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "opening_style", opening_style);
    AstNode::with_attr(&section_marker, "closing_style", closing_style);

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title, tokens_to_text(&tokens[start_at + 1..closing_index]));
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, closing_index + 1))
}

pub fn try_match_section_header_suffix(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let previous_line_start = move_back_one_line(tokens, start_at).unwrap_or(0);
    let closing_style = tokens[start_at].lexeme.clone();

    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "opening_style", "");
    AstNode::with_attr(&section_marker, "closing_style", closing_style);

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(
        &title,
        tokens_to_text(&tokens[previous_line_start..start_at]),
    );
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, start_at + 1))
}

fn find_next_newline(tokens: &[Token], start_at: usize) -> Option<usize> {
    tokens
        .iter()
        .enumerate()
        .skip(start_at)
        .find_map(|(index, token)| (token.kind == TokenKind::NewLine).then_some(index))
}

fn move_back_one_line(tokens: &[Token], index: usize) -> Option<usize> {
    // Move to the first token of the line ending before index
    let mut cursor = index.checked_sub(2)?;
    while !matches!(
        tokens[cursor].kind,
        TokenKind::NewLine | TokenKind::BlankLine
    ) {
        cursor = cursor.checked_sub(1)?;
    }
    Some(cursor + 1)
}

fn tokens_to_text(tokens: &[Token]) -> String {
    let mut text = String::new();
    for token in tokens {
        text.push_str(&token.lexeme);
    }
    text
}
