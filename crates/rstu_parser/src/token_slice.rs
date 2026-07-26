// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::ops::Range;
use std::sync::Arc;

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSliceError {
    TokenNotFound {
        kind: TokenKind,
        direction: ScanDirection,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenSlice {
    tokens: Arc<Vec<Token>>,
    start: usize,
    end: usize,
    cursor: usize,
}

impl TokenSlice {
    pub fn from_string(input: &str) -> Self {
        Self::from_tokens(tokenize(input))
    }

    pub fn from_tokens(tokens: Vec<Token>) -> Self {
        let len = tokens.len();
        Self {
            tokens: Arc::new(tokens),
            start: 0,
            end: len,
            cursor: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn cursor(&self) -> usize {
        self.cursor.saturating_sub(self.start)
    }

    pub fn set_cursor(&mut self, index: usize) -> bool {
        if index <= self.len() {
            self.cursor = self.start + index;
            true
        } else {
            false
        }
    }

    pub fn move_cursor(&mut self, direction: ScanDirection) -> bool {
        match direction {
            ScanDirection::Forward => {
                if self.cursor < self.end {
                    self.cursor += 1;
                    return true;
                }
            }
            ScanDirection::Backward => {
                if self.cursor > self.start {
                    self.cursor -= 1;
                    return true;
                }
            }
        }
        false
    }

    pub fn current(&self) -> Option<&Token> {
        self.tokens
            .get(self.cursor)
            .filter(|_| self.cursor < self.end)
    }

    pub fn get(&self, index: usize) -> Option<&Token> {
        (index < self.len()).then(|| &self.tokens[self.start + index])
    }

    pub fn as_slice(&self) -> &[Token] {
        &self.tokens[self.start..self.end]
    }

    pub fn slice(&self, range: Range<usize>) -> Option<Self> {
        if range.start > range.end || range.end > self.len() {
            return None;
        }

        let start = self.start + range.start;
        let end = self.start + range.end;
        Some(Self {
            tokens: self.tokens.clone(),
            start,
            end,
            cursor: start,
        })
    }

    pub fn to_text(&self) -> String {
        self.as_slice()
            .iter()
            .map(|token| token.lexeme.as_str())
            .collect()
    }

    pub fn until_next_kind(
        &self,
        kind: TokenKind,
        direction: ScanDirection,
    ) -> Result<Self, TokenSliceError> {
        let (start, end) = match direction {
            ScanDirection::Forward => {
                let found = self
                    .tokens
                    .iter()
                    .enumerate()
                    .skip(self.cursor)
                    .take(self.end.saturating_sub(self.cursor))
                    .find_map(|(index, token)| (token.kind == kind).then_some(index))
                    .ok_or(TokenSliceError::TokenNotFound { kind, direction })?;

                (self.cursor, found)
            }
            ScanDirection::Backward => {
                let found = self.tokens[self.start..self.cursor]
                    .iter()
                    .rposition(|token| token.kind == kind)
                    .map(|index| self.start + index + 1)
                    .ok_or(TokenSliceError::TokenNotFound { kind, direction })?;

                (found, self.cursor)
            }
        };

        Ok(Self {
            tokens: self.tokens.clone(),
            start,
            end,
            cursor: start,
        })
    }

    pub fn find_next_newline(&self, start_at: usize) -> Option<usize> {
        if start_at >= self.len() {
            return None;
        }

        self.slice(start_at..self.len()).and_then(|mut scan| {
            scan.set_cursor(0);
            let line = scan
                .until_next_kind(TokenKind::NewLine, ScanDirection::Forward)
                .ok()?;
            Some(start_at + line.len())
        })
    }

    pub fn move_back_one_line(&self, index: usize) -> Option<usize> {
        if index > self.len() {
            return None;
        }

        // Move to the first token of the line ending before index.
        let mut cursor = index.checked_sub(2)?;
        let token_values = self.as_slice();

        while !matches!(
            token_values[cursor].kind,
            TokenKind::NewLine | TokenKind::BlankLine
        ) {
            cursor = cursor.checked_sub(1)?;
        }

        Some(cursor + 1)
    }
}
