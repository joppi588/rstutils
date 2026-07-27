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
        kinds: Vec<TokenKind>,
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
    kinds: &[TokenKind],
    direction: ScanDirection,
    start_at: usize,
) -> Result<usize, TokenSliceError> {
    match direction {
        ScanDirection::Forward => tokens
            .iter()
            .enumerate()
            .skip(start_at)
            .find_map(|(index, token)| kinds.contains(&token.kind).then_some(index))
            .ok_or(TokenSliceError::TokenNotFound {
                kinds: kinds.to_vec(),
                direction,
            }),
        ScanDirection::Backward => tokens[..start_at]
            .iter()
            .rposition(|token| kinds.contains(&token.kind))
            .map(|index| index + 1)
            .ok_or(TokenSliceError::TokenNotFound {
                kinds: kinds.to_vec(),
                direction,
            }),
    }
}

#[cfg(test)]
mod tests {
    use super::{find_next_kind, ScanDirection};
    use crate::token::{Token, TokenKind};

    #[test]
    fn find_next_kind_matches_any_requested_kind() {
        let tokens = vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::NewLine, "\n"),
        ];

        let found = find_next_kind(
            &tokens,
            &[TokenKind::BlankLine, TokenKind::NewLine],
            ScanDirection::Forward,
            0,
        );

        assert_eq!(found, Ok(2));
    }

    #[test]
    fn find_next_kind_scans_backward_to_after_matching_token() {
        let tokens = vec![
            Token::new(TokenKind::Word, "before"),
            Token::new(TokenKind::BlankLine, "\n\n"),
            Token::new(TokenKind::Word, "after"),
            Token::new(TokenKind::Separator, "---"),
        ];

        let found = find_next_kind(
            &tokens,
            &[TokenKind::BlankLine, TokenKind::NewLine],
            ScanDirection::Backward,
            3,
        );

        assert_eq!(found, Ok(2));
    }
}
