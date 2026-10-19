// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenCategory, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenSliceError {
    TokenNotFound { kinds: Vec<TokenKind> },
    NoRemainingToken,
}

/// 1-based source position; columns count Unicode scalar values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self { line: 0, column: 1 } // 0 = leading synthetic blank line
    }
}

impl Position {
    pub fn advance(self, text: &str) -> Self {
        text.chars().fold(self, |position, character| {
            if character == '\n' {
                Self {
                    line: position.line + 1,
                    column: 1,
                }
            } else {
                Self {
                    column: position.column + 1,
                    ..position
                }
            }
        })
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "line {}, column {}", self.line, self.column)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenStream {
    tokens: Vec<(Token, Position)>,
    cursor: usize,
}

impl TokenStream {
    pub fn new(tokens: Vec<(Token, Position)>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn push(&mut self, token: Token, position: Position) {
        self.tokens.push((token, position));
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn from_pairs(tokens: &[(TokenKind, &str)]) -> Self {
        Self::new(
            tokens
                .iter()
                .map(|(kind, lexeme)| (Token::new(*kind, *lexeme), Position::default()))
                .collect(),
        )
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.iter().map(|(token, _)| token.clone()).collect()
    }

    /// Position of the token at `index`; past the end this is the last token's position.
    pub fn position_at(&self, index: usize) -> Position {
        self.tokens
            .get(index)
            .or_else(|| self.tokens.last())
            .map(|(_, position)| *position)
            .unwrap_or_default()
    }

    pub fn position_at_cursor(&self) -> Position {
        self.position_at(self.cursor)
    }

    pub fn consume_text_until(&mut self, kinds: &[TokenKind]) -> String {
        let found = self.find_next_kind(kinds).unwrap_or(self.tokens.len());
        let mut text = String::new();
        for (token, _) in &self.tokens[self.cursor.min(found)..found] {
            text.push_str(&token.lexeme);
        }
        self.cursor = found;
        text
    }

    pub fn consume_text_until_newline(&mut self) -> String {
        self.consume_text_until(&[TokenKind::NewLine])
    }

    // TODO: Remove
    /// Panic-free lookahead by absolute index, treating out-of-bounds reads as a synthetic `EoF` token.
    pub fn token_at(&self, index: usize) -> Token {
        self.tokens
            .get(index)
            .map(|(token, _)| token.clone())
            .unwrap_or_else(|| Token::new(TokenKind::EoF, ""))
    }

    pub fn token_at_cursor(&self) -> Token {
        self.token_at(self.cursor)
    }

    // TODO: Keep this or token_at, but not both
    pub fn token_peek_relative(&self, delta: isize) -> Token {
        let index = self.cursor.checked_add_signed(delta).unwrap_or(usize::MAX);
        self.token_at(index)
    }

    pub fn token_at_nextline(&self) -> Token {
        self.token_at(self.find_end_of_line() + 1)
    }

    pub fn find_end_of_line(&self) -> usize {
        self.find_next_kind(TokenCategory::EOL)
            .unwrap_or(self.tokens.len())
    }

    pub fn find_next_kind(&self, kinds: &[TokenKind]) -> Result<usize, TokenSliceError> {
        self.tokens[self.cursor..]
            .iter()
            .position(|(token, _)| token.kind.is(kinds))
            .map(|offset| offset + self.cursor)
            .ok_or(TokenSliceError::TokenNotFound {
                kinds: kinds.to_vec(),
            })
    }

    pub fn consume(&mut self) -> Token {
        let token = self.token_at(self.cursor);
        self.cursor = self.cursor.saturating_add(1);
        token
    }

    pub fn consume_newline(&mut self) -> Token {
        let token = self.consume();
        debug_assert_eq!(token.kind, TokenKind::NewLine);
        token
    }

    /// The inserted token takes the position of the token it displaces.
    pub fn insert_at(&mut self, index: usize, token: Token) {
        let position = self.position_at(index);
        self.tokens.insert(index, (token, position));
        if index <= self.cursor {
            self.cursor += 1;
        }
    }

    pub fn insert_at_cursor(&mut self, token: Token) {
        let cursor = self.cursor;
        self.insert_at(cursor, token);
        self.cursor = cursor;
    }

    pub fn update_at_cursor(&mut self, new_lexeme: String) {
        self.tokens[self.cursor].0.lexeme = new_lexeme;
    }

    pub fn take_at_cursor(&mut self) -> Token {
        self.take_at(self.cursor)
    }

    pub fn take_at(&mut self, index: usize) -> Token {
        let (token, _) = self.tokens.remove(index);
        if index < self.cursor {
            self.cursor -= 1;
        }
        token
    }
}

#[cfg(test)]
mod tests {
    use super::{Position, TokenStream};
    use crate::token::{Token, TokenKind};

    #[test]
    fn push_appends_token_with_position() {
        let mut stream = TokenStream::new(Vec::new());
        let position = Position { line: 2, column: 3 };

        stream.push(Token::new(TokenKind::Word, "a"), position);

        assert_eq!(stream.len(), 1);
        assert_eq!(stream.position_at(0), position);
    }

    #[test]
    fn kind_at_returns_eof_past_the_end() {
        let stream = TokenStream::from_pairs(&[(TokenKind::Word, "title")]);

        assert_eq!(stream.token_at(0).kind, TokenKind::Word);
        assert_eq!(stream.token_at(1).kind, TokenKind::EoF);
    }

    #[test]
    fn find_next_kind_matches_any_requested_kind() {
        let stream = TokenStream::from_pairs(&[
            (TokenKind::Word, "title"),
            (TokenKind::Spaces, " "),
            (TokenKind::NewLine, "\n"),
        ]);

        let found = stream.find_next_kind(&[TokenKind::BlankLine, TokenKind::NewLine]);

        assert_eq!(found, Ok(2));
    }

    #[test]
    fn kind_peek_relative_looks_ahead_of_the_cursor() {
        let stream =
            TokenStream::from_pairs(&[(TokenKind::Word, "title"), (TokenKind::NewLine, "\n")]);

        assert_eq!(stream.token_peek_relative(1).kind, TokenKind::NewLine);
        assert_eq!(stream.token_peek_relative(5).kind, TokenKind::EoF);
    }

    #[test]
    fn kind_at_nextline_returns_the_kind_after_the_next_newline() {
        let stream = TokenStream::from_pairs(&[
            (TokenKind::Word, "title"),
            (TokenKind::NewLine, "\n"),
            (TokenKind::Indent, "  "),
        ]);

        assert_eq!(stream.token_at_nextline().kind, TokenKind::Indent);
    }

    #[test]
    fn kind_at_nextline_returns_eof_when_no_newline_remains() {
        let stream = TokenStream::from_pairs(&[(TokenKind::Word, "title")]);

        assert_eq!(stream.token_at_nextline().kind, TokenKind::EoF);
    }

    #[test]
    fn consume_returns_the_token_and_advances_the_cursor() {
        let mut stream =
            TokenStream::from_pairs(&[(TokenKind::Word, "title"), (TokenKind::NewLine, "\n")]);

        assert_eq!(stream.consume(), Token::new(TokenKind::Word, "title"));
        assert_eq!(stream.cursor, 1);
        assert_eq!(stream.consume(), Token::new(TokenKind::NewLine, "\n"));
        assert_eq!(stream.cursor, 2);

        // Consuming past the end yields a synthetic EoF token.
        assert_eq!(stream.consume(), Token::new(TokenKind::EoF, ""));
        assert_eq!(stream.cursor, 3);
    }

    /// GIVEN text with newlines and multibyte characters
    /// WHEN a position is advanced over it
    /// THEN lines increment on newline and columns count characters
    #[test]
    fn position_advance_counts_characters_and_lines() {
        let position = Position::default().advance("\u{e9}a\nb\t");

        assert_eq!(position, Position { line: 1, column: 3 });
    }

    /// GIVEN a stream with a token inserted at the cursor
    /// WHEN positions are queried
    /// THEN the inserted token shares the displaced token's position
    #[test]
    fn inserted_token_takes_displaced_position() {
        let mut stream = TokenStream::new(vec![(
            Token::new(TokenKind::Word, "a"),
            Position { line: 4, column: 2 },
        )]);

        stream.insert_at_cursor(Token::indent(2));

        assert_eq!(stream.position_at(0), Position { line: 4, column: 2 });
        assert_eq!(stream.position_at(1), Position { line: 4, column: 2 });
    }

    /// GIVEN an empty stream or an index past the end
    /// WHEN a position is queried
    /// THEN the last known position (or 1:1) is returned
    #[test]
    fn position_past_the_end_uses_last_position() {
        let empty = TokenStream::new(vec![]);
        let stream = TokenStream::new(vec![(
            Token::new(TokenKind::EoF, ""),
            Position { line: 3, column: 1 },
        )]);

        assert_eq!(empty.position_at(5), Position::default());
        assert_eq!(stream.position_at(9), Position { line: 3, column: 1 });
    }
}
