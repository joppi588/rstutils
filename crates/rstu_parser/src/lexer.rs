// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind as TK};

macro_rules! space {
    ($n:expr) => {
        " ".repeat($n)
    };
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let input = format!("\n\n{input}\n\n"); // leading and trailing blank line
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_token_kind = TK::BlankLine;
    let mut current_indent = 0;

    let mut index: usize = 1;
    while index < input.len() - 1 {
        let sub_str = &input[index - 1..];
        let (token_kind, lexeme) = TK::match_token(sub_str)
            .unwrap_or_else(|| panic!("No token matched input: {sub_str:?}"));

        let new_token = Token::new(token_kind, lexeme);
        match (last_token_kind, token_kind) {
            // these special cases need more than one character context
            (TK::NewLine | TK::BlankLine, TK::Indent) => {
                let new_indent = lexeme.len();
                if new_indent > current_indent {
                    let indent_token = Token::new(TK::Indent, space!(new_indent - current_indent));
                    tokens.push(indent_token);
                } else if new_indent < current_indent {
                    let dedent_token = Token::new(TK::Dedent, space!(current_indent - new_indent));
                    tokens.push(dedent_token);
                }
                current_indent = new_indent;
            }
            (TK::NewLine | TK::BlankLine, TK::BlankLine) => tokens.push(new_token), // Blank line does not change indent
            (TK::NewLine | TK::BlankLine, _) => {
                if current_indent > 0 {
                    let dedent_token = Token::new(TK::Dedent, space!(current_indent));
                    tokens.push(dedent_token);
                }
                current_indent = 0;
                tokens.push(new_token);
            }
            (_, TK::BulletListMarker) => {
                tokens.push(
                    if last_token_kind.is(&[TK::Indent, TK::Dedent, TK::BlankLine, TK::NewLine]) {
                        new_token
                    } else {
                        Token::new(TK::Punctuation, lexeme)
                    },
                );
            }
            _ => tokens.push(new_token),
        }

        last_token_kind = token_kind;
        index += lexeme.len();
    }
    if current_indent > 0 {
        tokens.push(Token::new(TK::Dedent, space!(current_indent)))
    };
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn tokenize_spaces_words() {
        let input = "Hello World\n";
        let expected = vec![
            Token::new(TK::BlankLine, "\n"),
            Token::new(TK::Word, "Hello"),
            Token::new(TK::Spaces, " "),
            Token::new(TK::Word, "World"),
            Token::new(TK::NewLine, "\n"),
            Token::new(TK::BlankLine, "\n"),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "abc\x07def\n";
        let expected = vec![
            Token::new(TK::BlankLine, "\n"),
            Token::new(TK::Word, "abc"),
            Token::new(TK::LiteralChar, "\x07"),
            Token::new(TK::Word, "def"),
            Token::new(TK::NewLine, "\n"),
            Token::new(TK::BlankLine, "\n"),
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
                (TK::BlankLine, "\n"),
                (TK::Word, "line_1"),
                (TK::NewLine, "\n"),
                (TK::Indent, "    "),
                (TK::Word, "nested"),
                (TK::NewLine, "\n"),
                (TK::Dedent, "  "),
                (TK::Word, "dedented"),
                (TK::NewLine, "\n"),
                (TK::BlankLine, "\n"),
                (TK::Dedent, "  ")
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
                (TK::BlankLine, "\n"),
                (TK::Word, "line_1"),
                (TK::NewLine, "\n"),
                (TK::Indent, "    "),
                (TK::Word, "nested"),
                (TK::NewLine, "\n"),
                (TK::Dedent, "    "),
                (TK::Word, "plain"),
                (TK::NewLine, "\n"),
                (TK::BlankLine, "\n"),
            ]
        );
    }
}
