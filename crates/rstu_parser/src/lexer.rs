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
        tokens.push(Token::new(kind, lexeme));
        index += lexeme.len();
    }

    let mut transformed = Vec::new();
    let mut current_indent: usize = 0;

    transformed.push(tokens[0].clone());
    index = 1;
    while index < tokens.len() {
        match (tokens[index - 1].kind, tokens[index].kind) {
            (TokenKind::NewLine, TokenKind::Indent) => {
                let new_indent = tokens[index].lexeme.len();
                if new_indent > current_indent {
                    let indent_token =
                        Token::new(TokenKind::Indent, " ".repeat(new_indent - current_indent));
                    transformed.push(indent_token);
                } else if new_indent < current_indent {
                    let dedent_token =
                        Token::new(TokenKind::Dedent, " ".repeat(current_indent - new_indent));
                    transformed.push(dedent_token);
                }
                current_indent = new_indent;
            }
            (TokenKind::NewLine, _) => {
                if current_indent > 0 {
                    let dedent = Token::new(TokenKind::Dedent, " ".repeat(current_indent));
                    transformed.push(dedent);
                }
                current_indent = 0;
                transformed.push(tokens[index].clone());
            }
            _ => transformed.push(tokens[index].clone()),
        }
        index += 1;
    }
    transformed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

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

    #[test]
    fn tokenize_converts_indents_to_relative_indents_and_dedents() {
        let input = "line_1\n    nested\n  dedented\n";
        let tokens = tokenize(input);
        let actual: Vec<(TokenKind, &str)> = tokens
            .iter()
            .map(|token| (token.kind, token.lexeme.as_str()))
            .collect();

        assert_eq!(
            actual,
            vec![
                (TokenKind::BlankLine, "\n"),
                (TokenKind::Word, "line_1"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::Indent, "    "),
                (TokenKind::Word, "nested"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::Dedent, "  "),
                (TokenKind::Word, "dedented"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::Dedent, "  "),
                (TokenKind::BlankLine, "\n"),
            ]
        );
    }

    #[test]
    fn tokenize_emits_dedent_when_indented_block_returns_to_zero_indent() {
        let input = "line_1\n    nested\nplain\n";
        let tokens = tokenize(input);
        let actual: Vec<(TokenKind, &str)> = tokens
            .iter()
            .map(|token| (token.kind, token.lexeme.as_str()))
            .collect();

        assert_eq!(
            actual,
            vec![
                (TokenKind::BlankLine, "\n"),
                (TokenKind::Word, "line_1"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::Indent, "    "),
                (TokenKind::Word, "nested"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::Dedent, "    "),
                (TokenKind::Word, "plain"),
                (TokenKind::NewLine, "\n"),
                (TokenKind::BlankLine, "\n"),
            ]
        );
    }
}
