// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

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

pub fn tokens_to_text(tokens: &[Token]) -> String {
    let mut text = String::new();
    for token in tokens {
        text.push_str(&token.lexeme);
    }
    text
}

pub fn find_next_kind(
    tokens: &[Token],
    kind: TokenKind,
    direction: ScanDirection,
    start_at: usize,
) -> Result<usize, TokenSliceError> {
    match direction {
        ScanDirection::Forward => tokens
            .iter()
            .enumerate()
            .skip(start_at)
            .find_map(|(index, token)| (token.kind == kind).then_some(index))
            .ok_or(TokenSliceError::TokenNotFound { kind, direction }),
        ScanDirection::Backward => tokens[..start_at]
            .iter()
            .rposition(|token| token.kind == kind)
            .map(|index| start_at + index + 1)
            .ok_or(TokenSliceError::TokenNotFound { kind, direction }),
    }
}
