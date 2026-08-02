// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

fn whitespace_width(lexeme: &str) -> usize {
    lexeme.chars().count()
}

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
    let mut current_indent: Option<(String, usize)> = None;

    for token in tokens {
        if token.kind != TokenKind::Indent {
            if let Some((previous_lexeme, previous_width)) = current_indent.as_ref() {
                if *previous_width > 0 {
                    let dedent = Token::new(TokenKind::Dedent, previous_lexeme.clone());
                    transformed.push(dedent);
                }
                current_indent = None;
            }
            transformed.push(token);
            continue;
        }

        let indent_width = whitespace_width(&token.lexeme);
        let relative_token = match current_indent.as_ref() {
            None => {
                current_indent = Some((token.lexeme.clone(), indent_width));
                Some(Token::new(TokenKind::Indent, token.lexeme.clone()))
            }
            Some((previous_lexeme, previous_width)) => {
                if indent_width > *previous_width {
                    let delta = indent_width - previous_width;
                    let relative_lexeme = token
                        .lexeme
                        .chars()
                        .skip(token.lexeme.chars().count().saturating_sub(delta))
                        .collect::<String>();
                    current_indent = Some((token.lexeme.clone(), indent_width));
                    Some(Token::new(TokenKind::Indent, relative_lexeme))
                } else if indent_width < *previous_width {
                    let delta = previous_width - indent_width;
                    let relative_lexeme = previous_lexeme
                        .chars()
                        .skip(previous_lexeme.chars().count().saturating_sub(delta))
                        .collect::<String>();
                    current_indent = Some((token.lexeme.clone(), indent_width));
                    Some(Token::new(TokenKind::Dedent, relative_lexeme))
                } else {
                    current_indent = Some((token.lexeme.clone(), indent_width));
                    None
                }
            }
        };

        if let Some(relative_token) = relative_token {
            transformed.push(relative_token);
        }
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
        let input = "line 1\n    nested\n  dedented\n";
        let tokens = tokenize(input);
        let actual: Vec<(TokenKind, &str)> = tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Indent || token.kind == TokenKind::Dedent)
            .map(|token| (token.kind, token.lexeme.as_str()))
            .collect();

        assert_eq!(
            actual,
            vec![
                (TokenKind::Indent, "    "),
                (TokenKind::Dedent, "    "),
                (TokenKind::Indent, "  "),
                (TokenKind::Dedent, "  "),
            ]
        );
    }

    #[test]
    fn tokenize_emits_dedent_when_indented_block_returns_to_zero_indent() {
        let input = "line 1\n    nested\nplain\n";
        let tokens = tokenize(input);
        let actual: Vec<(TokenKind, &str)> = tokens
            .iter()
            .filter(|token| token.kind == TokenKind::Indent || token.kind == TokenKind::Dedent)
            .map(|token| (token.kind, token.lexeme.as_str()))
            .collect();

        assert_eq!(
            actual,
            vec![(TokenKind::Indent, "    "), (TokenKind::Dedent, "    ")]
        );
    }
}
