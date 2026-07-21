// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use rstu_ast::{ElementKind, Node};

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindElementError {
    StartAtOutOfBounds { start_at: usize, token_count: usize },
    SectionTitleMissingNextLineAfterOpening { opening_index: usize },
    SectionTitleMissingClosingAfterOpening { opening_index: usize },
    SectionTitleMissingPreviousLineBeforeClosing { closing_index: usize },
}

pub fn parse(input: &str) -> Node {
    let _tokens = tokenize(input);
    let doc: Node = Node::new(ElementKind::Document);

    doc
}

pub fn try_find_section_header(
    tokens: &[Token],
    start_at: usize,
) -> Result<Option<(usize, usize)>, FindElementError> {
    for index in start_at..tokens.len() {
        match tokens[index].kind {
            TokenKind::SectionTitlePrefix => {
                let next_line_end = find_next_newline(tokens, index + 1).ok_or(
                    FindElementError::SectionTitleMissingNextLineAfterOpening {
                        opening_index: index,
                    },
                )?;

                if next_line_end + 1 >= tokens.len()
                    || tokens[next_line_end + 1].kind != TokenKind::SectionTitleSuffix
                {
                    return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
                        opening_index: index,
                    });
                }

                return Ok(Some((index, next_line_end + 1)));
            }
            TokenKind::SectionTitleSuffix => {
                let Some(previous_line_start) = move_back_one_line(tokens, index) else {
                    return Err(
                        FindElementError::SectionTitleMissingPreviousLineBeforeClosing {
                            closing_index: index,
                        },
                    );
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

fn move_back_one_line(tokens: &[Token], index: usize) -> Option<usize> {
    let mut cursor = index.checked_sub(2)?;
    while cursor > 0 && tokens[cursor - 1].kind != TokenKind::NewLine {
        cursor -= 1;
    }

    while matches!(
        tokens[cursor].kind,
        TokenKind::NewLine | TokenKind::BlankLine
    ) {
        cursor += 1;
    }

    Some(cursor)
}
