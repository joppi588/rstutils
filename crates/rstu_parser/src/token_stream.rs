// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSliceError {
    TokenNotFound { kinds: Vec<TokenKind> },
    NoRemainingToken,
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

pub fn find_next_kind(
    tokens: &[Token],
    kinds: &[TokenKind],
    start_at: usize,
) -> Result<usize, TokenSliceError> {
    Ok(find_next_kind_interrupt(tokens, kinds, &[], start_at)?
        .expect("interrupt_kinds is empty, so None is unreachable"))
}

// TODO: remove if not used finally

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

/// A token buffer paired with a cursor, so parsers no longer thread an index through return values.
pub struct TokenStream {
    tokens: Vec<Token>,
    cursor: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn is_at_end(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    /// Jumps the cursor to an absolute position, e.g. after slicing tokens by index.
    pub fn set_cursor(&mut self, pos: usize) {
        self.cursor = pos;
    }

    /// Panic-free lookahead by absolute index, treating out-of-bounds reads as `TokenKind::EoF`.
    pub fn kind_at(&self, index: usize) -> TokenKind {
        self.tokens
            .get(index)
            .map_or(TokenKind::EoF, |token| token.kind)
    }

    pub fn kind_at_cursor(&self) -> TokenKind {
        self.kind_at(self.cursor)
    }

    pub fn kind_peek_relative(&self, delta: usize) -> TokenKind {
        self.kind_at(self.cursor.saturating_add(delta))
    }

    pub fn kind_at_nextline(&self) -> TokenKind {
        let line_end = find_next_kind(
            &self.tokens,
            &[TokenKind::NewLine, TokenKind::BlankLine, TokenKind::EoF],
            self.cursor,
        )
        .unwrap_or(self.tokens.len());
        self.kind_at(line_end + 1)
    }

    pub fn find_next_kind(&self, kinds: &[TokenKind]) -> Result<usize, TokenSliceError> {
        find_next_kind(&self.tokens, kinds, self.cursor)
    }

    pub fn token_at_nextline(&self) -> Token {
        let line_end = find_next_kind(
            &self.tokens,
            &[TokenKind::NewLine, TokenKind::BlankLine, TokenKind::EoF],
            self.cursor,
        )
        .unwrap_or(self.tokens.len());
        self.tokens
            .get(line_end + 1)
            .cloned()
            .unwrap_or_else(|| Token::new(TokenKind::EoF, ""))
    }

    /// Returns the token at the cursor (or a synthetic `EoF` token) and advances the cursor by one.
    pub fn consume(&mut self) -> Token {
        let token = self
            .tokens
            .get(self.cursor)
            .cloned()
            .unwrap_or_else(|| Token::new(TokenKind::EoF, ""));
        self.cursor = self.cursor.saturating_add(1).min(self.tokens.len());
        token
    }

    pub fn consume_n(&mut self, n: usize) {
        self.cursor = self.cursor.saturating_add(n).min(self.tokens.len());
    }
}

#[cfg(test)]
mod tests {
    use super::{find_next_kind, tokens_without_kinds, TokenStream};
    use crate::token::{Token, TokenKind};

    #[test]
    fn kind_at_returns_eof_past_the_end() {
        let stream = TokenStream::new(vec![Token::new(TokenKind::Word, "title")]);

        assert_eq!(stream.kind_at(0), TokenKind::Word);
        assert_eq!(stream.kind_at(1), TokenKind::EoF);
    }

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
    fn kind_at_cursor_returns_eof_past_the_end() {
        let mut stream = TokenStream::new(vec![Token::new(TokenKind::Word, "title")]);

        assert_eq!(stream.kind_at_cursor(), TokenKind::Word);

        stream.consume_n(1);
        assert_eq!(stream.kind_at_cursor(), TokenKind::EoF);
    }

    #[test]
    fn kind_peek_relative_looks_ahead_of_the_cursor() {
        let stream = TokenStream::new(vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::NewLine, "\n"),
        ]);

        assert_eq!(stream.kind_peek_relative(1), TokenKind::NewLine);
        assert_eq!(stream.kind_peek_relative(5), TokenKind::EoF);
    }

    #[test]
    fn kind_at_nextline_returns_the_kind_after_the_next_newline() {
        let stream = TokenStream::new(vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::NewLine, "\n"),
            Token::new(TokenKind::Indent, "  "),
        ]);

        assert_eq!(stream.kind_at_nextline(), TokenKind::Indent);
    }

    #[test]
    fn kind_at_nextline_returns_eof_when_no_newline_remains() {
        let stream = TokenStream::new(vec![Token::new(TokenKind::Word, "title")]);

        assert_eq!(stream.kind_at_nextline(), TokenKind::EoF);
    }

    #[test]
    fn consume_returns_the_token_and_advances_the_cursor() {
        let mut stream = TokenStream::new(vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::NewLine, "\n"),
        ]);

        assert_eq!(stream.consume(), Token::new(TokenKind::Word, "title"));
        assert_eq!(stream.cursor(), 1);
        assert_eq!(stream.consume(), Token::new(TokenKind::NewLine, "\n"));
        assert_eq!(stream.cursor(), 2);
        assert!(stream.is_at_end());

        // Consuming past the end stays at the boundary and yields a synthetic EoF token.
        assert_eq!(stream.consume(), Token::new(TokenKind::EoF, ""));
        assert_eq!(stream.cursor(), 2);
    }

    #[test]
    fn consume_n_advances_the_cursor_by_n_and_saturates() {
        let mut stream = TokenStream::new(vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::NewLine, "\n"),
        ]);

        stream.consume_n(1);
        assert_eq!(stream.cursor(), 1);

        stream.consume_n(10);
        assert_eq!(stream.cursor(), 2);
    }
}
