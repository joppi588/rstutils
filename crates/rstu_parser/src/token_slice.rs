// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use std::ops::Range;
use std::sync::Arc;

use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

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

    pub fn advance(&mut self) -> bool {
        if self.cursor < self.end {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    pub fn retreat(&mut self) -> bool {
        if self.cursor > self.start {
            self.cursor -= 1;
            true
        } else {
            false
        }
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

    pub fn until_next_kind_forward(&self, kind: TokenKind) -> Self {
        let end = self
            .tokens
            .iter()
            .enumerate()
            .skip(self.cursor)
            .take(self.end.saturating_sub(self.cursor))
            .find_map(|(index, token)| (token.kind == kind).then_some(index))
            .unwrap_or(self.end);

        Self {
            tokens: self.tokens.clone(),
            start: self.cursor,
            end,
            cursor: self.cursor,
        }
    }

    pub fn until_next_kind_backward(&self, kind: TokenKind) -> Self {
        let from = self.cursor.min(self.end);
        let start = self.tokens[self.start..from]
            .iter()
            .rposition(|token| token.kind == kind)
            .map(|index| self.start + index + 1)
            .unwrap_or(self.start);

        Self {
            tokens: self.tokens.clone(),
            start,
            end: from,
            cursor: start,
        }
    }
}
