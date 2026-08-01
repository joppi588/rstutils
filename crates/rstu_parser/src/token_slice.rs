// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSliceError {
    TokenNotFound { kinds: Vec<TokenKind> },
}

pub fn tokens_to_text(tokens: &[Token]) -> String {
    let mut text = String::new();
    for token in tokens {
        text.push_str(&token.lexeme);
    }
    text
}

pub fn tokens_without_kinds(tokens: &[Token], kinds: &[TokenKind]) -> Vec<Token> {
    tokens
        .iter()
        .filter(|token| !kinds.contains(&token.kind))
        .cloned()
        .collect()
}

pub fn trim_leading_kinds(tokens: &[Token], kinds: &[TokenKind]) -> Vec<Token> {
    let prefix_len = tokens
        .iter()
        .take_while(|token| kinds.contains(&token.kind))
        .count();
    tokens[prefix_len..].to_vec()
}

// better have two separate finders, one with interrupt, the other without.

pub fn find_next_kind(
    tokens: &[Token],
    kinds: &[TokenKind],
    start_at: usize,
) -> Result<usize, TokenSliceError> {
    Ok(find_next_kind_interrupt(tokens, kinds, &[], start_at)?
        .expect("interrupt_kinds is empty, so None is unreachable"))
}

pub fn find_next_kind_interrupt(
    tokens: &[Token],
    kinds: &[TokenKind],
    interrupt_kinds: &[TokenKind],
    start_at: usize,
) -> Result<Option<usize>, TokenSliceError> {
    tokens
        .iter()
        .enumerate()
        .skip(start_at)
        .find_map(|(index, token)| {
            if (&token.kind).is(kinds) {
                return Some(Some(index));
            }
            if (&token.kind).is(interrupt_kinds) {
                return Some(None);
            }
            None
        })
        .ok_or(TokenSliceError::TokenNotFound {
            kinds: kinds.to_vec(),
        })
}

pub fn skip_kinds(
    tokens: &[Token],
    kinds: &[TokenKind],
    start_at: usize,
) -> Result<usize, TokenSliceError> {
    tokens
        .iter()
        .enumerate()
        .skip(start_at)
        .find_map(|(index, token)| (!kinds.contains(&token.kind)).then_some(index))
        .ok_or(TokenSliceError::TokenNotFound {
            kinds: kinds.to_vec(),
        })
}

#[cfg(test)]
mod tests {
    use super::{find_next_kind, skip_kinds, tokens_without_kinds, trim_leading_kinds};
    use crate::token::{Token, TokenKind};

    #[test]
    fn find_next_kind_matches_any_requested_kind() {
        let tokens = vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::NewLine, "\n"),
        ];

        let found = find_next_kind(&tokens, &[TokenKind::BlankLine, TokenKind::NewLine], 0);

        assert_eq!(found, Ok(2));
    }

    #[test]
    fn skip_kinds_returns_first_non_matching_token_index() {
        let tokens = vec![
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::NewLine, "\n"),
            Token::new(TokenKind::Word, "title"),
        ];

        let found = skip_kinds(&tokens, &[TokenKind::Spaces, TokenKind::NewLine], 0);

        assert_eq!(found, Ok(2));
    }

    #[test]
    fn skip_kinds_fails_when_remaining_tokens_all_match() {
        let tokens = vec![
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::NewLine, "\n"),
        ];

        let found = skip_kinds(&tokens, &[TokenKind::Spaces, TokenKind::NewLine], 0);

        assert_eq!(
            found,
            Err(super::TokenSliceError::TokenNotFound {
                kinds: vec![TokenKind::Spaces, TokenKind::NewLine],
            })
        );
    }

    #[test]
    fn tokens_without_kinds_removes_requested_token_kinds() {
        let tokens = vec![
            Token::new(TokenKind::Indent, "   "),
            Token::new(TokenKind::Word, "hello"),
            Token::new(TokenKind::Indent, "   "),
            Token::new(TokenKind::Punctuation, "."),
        ];

        let filtered = tokens_without_kinds(&tokens, &[TokenKind::Indent]);

        assert_eq!(
            filtered,
            vec![
                Token::new(TokenKind::Word, "hello"),
                Token::new(TokenKind::Punctuation, "."),
            ]
        );
    }

    #[test]
    fn trim_leading_kinds_removes_requested_prefix_tokens() {
        let tokens = vec![
            Token::new(TokenKind::DoubleDot, ".."),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::Word, "hello"),
        ];

        let trimmed = trim_leading_kinds(&tokens, &[TokenKind::DoubleDot, TokenKind::Spaces]);

        assert_eq!(trimmed, vec![Token::new(TokenKind::Word, "hello")]);
    }
}
