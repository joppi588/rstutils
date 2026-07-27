// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;
pub mod token_slice;

use rstu_ast::{AstNode, ElementKind, NodeRef};

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};
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
}

pub fn parse(input: &str) -> Result<NodeRef, FindElementError> {
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(ElementKind::Document);
    let mut index: usize = 0;
    let mut current_node = doc.clone();

    while index < tokens.len() - 1 {
        let index_line_end = find_next_kind(
            &tokens,
            &[TokenKind::NewLine],
            ScanDirection::Forward,
            index,
        )
        .expect("Token stream shall end with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (TokenKind::Separator, TokenKind::Indent) | (TokenKind::Separator, TokenKind::Word) => {
                let (section, next_start) = try_match_section_header_prefix(&tokens, index)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = next_start;
            }
            (TokenKind::Word, TokenKind::Separator) => {
                let (section, index_end_header) = try_match_section_header_suffix(&tokens, index)?;
                AstNode::push_section_ref(&current_node, section.clone())
                    .expect("Could not insert section!");
                current_node = section;
                index = index_end_header + 1;
            }

            _ => index += 1,
        };
    }

    Ok(doc)
}

pub fn try_match_section_header_prefix(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let next_line_end = find_next_kind(
        tokens,
        &[TokenKind::NewLine],
        ScanDirection::Forward,
        start_at + 2,
    )
    .map_err(
        |_| FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        },
    )?;

    let closing_index = next_line_end + 1;
    if (closing_index >= tokens.len()) || (tokens[closing_index].kind != TokenKind::Separator) {
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
    AstNode::with_attr(&section_marker, "section_marker", closing_style);

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(
        &title,
        tokens_to_text(&tokens[start_at + 1..next_line_end + 1]),
    );
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, closing_index + 1))
}

pub fn try_match_section_header_suffix(
    tokens: &Vec<Token>,
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let line_end = find_next_kind(
        tokens,
        &[TokenKind::NewLine],
        ScanDirection::Forward,
        start_at,
    )
    .expect("Section title ends with new line.");
    let closing_style = tokens[line_end + 1].lexeme.clone();

    let section_marker = AstNode::new_ref(ElementKind::Section);
    AstNode::with_attr(&section_marker, "section_marker", closing_style);

    let title = AstNode::new_ref(ElementKind::Title);
    AstNode::with_text(&title, tokens_to_text(&tokens[start_at..line_end + 1]));
    AstNode::push_child(&section_marker, title)
        .expect("section title should always be a valid section child");

    Ok((section_marker, line_end + 2))
}
