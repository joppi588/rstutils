// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use std::collections::BTreeMap;
use std::thread::current;

use rstu_ast::{ElementKind, Node};

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

pub fn parse(input: &str) -> Node {
    let tokens = tokenize(input);
    let doc: Node = Node::new(ElementKind::Document);
    let mut current_node = &doc;

    for index in tokens {
        if Some(section_header) = try_match_section_header(tokens,index) {
            current_node.push_section(section_header)
        }
    }

    doc
}

pub fn try_match_section_header(
    tokens: &[Token],
    start_at: usize,
) -> Result<Option<&Node>, FindElementError> {
        match tokens[0].kind {
            TokenKind::SectionTitlePrefix => {
                let next_line_end = find_next_newline(tokens, start_at+2).ok_or(
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

                let opening_style =tokens[start_at].lexeme.clone(); // TODO: single char + length
                let closing_style = tokens[closing_index].lexeme.clone();
                if tokens[index].lexeme != tokens[closing_index].lexeme {
                    return Err(FindElementError::SectionTitleUnbalancedStyle {
                        opening_index: start_at,
                        opening_style: opening_style,
                        closing_style: closing_style,
                    });
                }
                let section_marker = Node::new(ElementKind::Section)
                    .with_attr("opening_style", opening_style)
                    .with_attr("closing_style",closing_style )
                    .with_child(Node::new(ElementKind::Title)
                        .with_text(tokens_to_text(&tokens[start_index+1..closing_index])));

                return Ok(Some(section_marker));
            }
            TokenKind::SectionTitleSuffix => {
                let previous_line_start = move_back_one_line(tokens, index).unwrap_or(0);
                let section_marker = Node::new(ElementKind::Section)
                    .with_attr("opening_style", "")
                    .with_attr("closing_style",closing_style )
                    .with_child(Node::new(ElementKind::Title)
                        .with_text(tokens_to_text(&tokens[previous_line_start..closing_index])));

                return Ok(Some(section_marker));
            }
            _ => {}
        }
    }

    Ok(None)
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
