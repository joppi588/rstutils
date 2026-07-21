// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use rstu_ast::{ElementKind, Node};

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindSectionHeaderError {
    StartAtOutOfBounds { start_at: usize, token_count: usize },
    MissingNextLineAfterOpening { opening_index: usize },
    MissingClosingAfterOpening { opening_index: usize },
    MissingPreviousLineBeforeClosing { closing_index: usize },
}

pub fn parse(input: &str) -> Node {
    let tokens = tokenize(input);
    let doc: Node = Node::new(ElementKind::Document);

    doc
}

pub fn try_find_section_header(
    tokens: &[Token],
    start_at: usize,
) -> Result<Option<(usize, usize)>, FindSectionHeaderError> {
    if start_at > tokens.len() {
        return Err(FindSectionHeaderError::StartAtOutOfBounds {
            start_at,
            token_count: tokens.len(),
        });
    }

    for index in start_at..tokens.len() {
        match tokens[index].kind {
            TokenKind::SectionTitlePrefix => {
                let Some(next_line_end) = find_next_newline(tokens, index + 1) else {
                    return Err(FindSectionHeaderError::MissingNextLineAfterOpening {
                        opening_index: index,
                    });
                };

                let closing_index = next_line_end + 1;
                if closing_index >= tokens.len()
                    || tokens[closing_index].kind != TokenKind::SectionTitleSuffix
                {
                    return Err(FindSectionHeaderError::MissingClosingAfterOpening {
                        opening_index: index,
                    });
                }

                return Ok(Some((index, closing_index)));
            }
            TokenKind::SectionTitleSuffix => {
                let Some(previous_line_start) = find_previous_line_start(tokens, index) else {
                    return Err(FindSectionHeaderError::MissingPreviousLineBeforeClosing {
                        closing_index: index,
                    });
                };

                return Ok(Some((previous_line_start, index)));
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

fn find_previous_line_start(tokens: &[Token], closing_index: usize) -> Option<usize> {
    let mut cursor = closing_index.checked_sub(1)?;
    if tokens[cursor].kind == TokenKind::NewLine {
        cursor = cursor.checked_sub(1)?;
    }

    while cursor > 0 && tokens[cursor - 1].kind != TokenKind::NewLine {
        cursor -= 1;
    }

    Some(cursor)
}
