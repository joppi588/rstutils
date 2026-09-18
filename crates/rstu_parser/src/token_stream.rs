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
        let line_end = self
            .find_next_kind(&[TokenKind::NewLine, TokenKind::BlankLine, TokenKind::EoF])
            .unwrap_or(self.tokens.len());
        self.kind_at(line_end + 1)
    }

    pub fn find_next_kind(&self, kinds: &[TokenKind]) -> Result<usize, TokenSliceError> {
        self.find_next_kind_from(kinds, self.cursor)
    }

    pub fn find_next_kind_from(
        &self,
        kinds: &[TokenKind],
        start_at: usize,
    ) -> Result<usize, TokenSliceError> {
        self.tokens
            .iter()
            .enumerate()
            .skip(start_at)
            .find(|(_, token)| token.kind.is(kinds))
            .map(|(index, _)| index)
            .ok_or(TokenSliceError::TokenNotFound {
                kinds: kinds.to_vec(),
            })
    }

    pub fn token_at_nextline(&self) -> Token {
        let line_end = self
            .find_next_kind(&[TokenKind::NewLine, TokenKind::BlankLine, TokenKind::EoF])
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

    /// Inserts a token at an absolute index, shifting the cursor if it lies at or after the insertion point.
    pub fn insert_at(&mut self, index: usize, token: Token) {
        self.tokens.insert(index, token);
        if index <= self.cursor {
            self.cursor += 1;
        }
    }

    /// Removes and returns the token at an absolute index, shifting the cursor if it precedes it.
    pub fn take_at(&mut self, index: usize) -> Token {
        let token = self.tokens.remove(index);
        if index < self.cursor {
            self.cursor -= 1;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::{tokens_without_kinds, TokenStream};
    use crate::token::{Token, TokenKind};

    #[test]
    fn kind_at_returns_eof_past_the_end() {
        let stream = TokenStream::new(vec![Token::new(TokenKind::Word, "title")]);

        assert_eq!(stream.kind_at(0), TokenKind::Word);
        assert_eq!(stream.kind_at(1), TokenKind::EoF);
    }

    #[test]
    fn find_next_kind_matches_any_requested_kind() {
        let stream = TokenStream::new(vec![
            Token::new(TokenKind::Word, "title"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::NewLine, "\n"),
        ]);

        let found = stream.find_next_kind_from(&[TokenKind::BlankLine, TokenKind::NewLine], 0);

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
