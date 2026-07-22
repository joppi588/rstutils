// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rst_parser::token;

use crate::token::{Token, TokenKind};

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = format!("\n\n{input}\n\n"); // leading and trailing blank line
    let mut index: usize = 2;
    while index < input.len() - 2 {
        let mut lexeme_len = 0;
        for kind in TokenKind::ALL {
            let sub_str = &input[index - kind.context_len().0..];
            if let Some(token_match) = kind.find(sub_str) {
                lexeme_len = token_match.len() - kind.context_len().0 - kind.context_len().1 + 1;
                tokens.push(Token::new(
                    kind,
                    &sub_str[(kind.context_len().0)..lexeme_len],
                ));
                break;
            }
        }
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
            Token::new(TokenKind::Word, "Hello"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::Word, "World"),
            Token::new(TokenKind::NewLine, "\n"),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "*%*%*\n";
        let expected = vec![
            Token::new(TokenKind::LiteralChar, "*"),
            Token::new(TokenKind::LiteralChar, "%"),
            Token::new(TokenKind::LiteralChar, "*"),
            Token::new(TokenKind::LiteralChar, "%"),
            Token::new(TokenKind::LiteralChar, "*"),
            Token::new(TokenKind::NewLine, "\n"),
        ];

        assert_eq!(tokenize(input), expected);
    }
}
