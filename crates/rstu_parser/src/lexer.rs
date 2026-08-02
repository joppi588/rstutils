// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = format!("\n\n{input}\n\n"); // leading and trailing blank line
    let mut index: usize = 1;
    while index < input.len() - 1 {
        let sub_str = &input[index - 1..];
        let (kind, lexeme) = TokenKind::match_token(sub_str)
            .unwrap_or_else(|| panic!("No token matched input: {sub_str:?}"));
        let lexeme_len = lexeme.len() - 2;
        tokens.push(Token::new(kind, &sub_str[1..lexeme_len + 1]));
        index += lexeme_len;
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_spaces_words() {
        let input = "Hello World\n";
        let expected = vec![
            Token::new(TokenKind::BlankLine, "\n"),
            Token::new(TokenKind::Word, "Hello"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::Word, "World"),
            Token::new(TokenKind::NewLine, "\n"),
            Token::new(TokenKind::BlankLine, "\n"),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "abc\x07def\n";
        let expected = vec![
            Token::new(TokenKind::BlankLine, "\n"),
            Token::new(TokenKind::Word, "abc"),
            Token::new(TokenKind::LiteralChar, "\x07"),
            Token::new(TokenKind::Word, "def"),
            Token::new(TokenKind::NewLine, "\n"),
            Token::new(TokenKind::BlankLine, "\n"),
        ];

        assert_eq!(tokenize(input), expected);
    }
}
