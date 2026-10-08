// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;

pub fn tokenize(input: &str) -> TokenStream {
    let input = format!("\n{input}\n"); // leading blank line
    let mut tokens: Vec<Token> = Vec::new();
    let mut last_token_kind = TK::BlankLine;
    let mut current_indent = 0;

    let mut index: usize = 1;
    while index < input.len() - 1 {
        let sub_str = &input[index - 1..];
        let (token_kind, lexeme) = TK::match_token(sub_str)
            .unwrap_or_else(|| panic!("No token matched input: {sub_str:?}"));

        let new_token = Token::new(token_kind, lexeme);
        match (token_kind, last_token_kind) {
            (TK::BlankLine, _) => tokens.push(new_token),
            (TK::Indent, _) => {
                let new_indent = lexeme.len();
                if new_indent > current_indent {
                    let indent_token = Token::indent(new_indent - current_indent);
                    tokens.push(indent_token);
                } else if new_indent < current_indent {
                    let dedent_token = Token::dedent(current_indent - new_indent);
                    if last_token_kind == TK::BlankLine {
                        tokens.insert(tokens.len() - 1, dedent_token);
                    } else {
                        tokens.push(dedent_token);
                    }
                }
                current_indent = new_indent;
            }
            (_, TK::NewLine) => {
                // Not indented
                if current_indent > 0 {
                    let dedent_token = Token::dedent(current_indent);
                    tokens.push(dedent_token);
                }
                current_indent = 0;
                tokens.push(new_token);
            }
            (_, TK::BlankLine) => {
                // Not indented, insert dedent before blankline
                if current_indent > 0 {
                    let dedent_token = Token::dedent(current_indent);
                    tokens.insert(tokens.len() - 1, dedent_token);
                }
                current_indent = 0;
                tokens.push(new_token);
            }

            _ => tokens.push(new_token),
        }

        last_token_kind = token_kind;
        index += lexeme.len();
    }
    if current_indent > 0 {
        tokens.push(Token::dedent(current_indent))
    };
    tokens.push(Token::new(TK::EoF, ""));
    TokenStream::new(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_stream::TokenStream;

    #[test]
    fn tokenize_spaces_words() {
        let input = "Hello World\n";
        let expected = TokenStream::from_pairs(&[
            (TK::Word, "Hello"),
            (TK::Spaces, " "),
            (TK::Word, "World"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_option_no_newline() {
        // GIVEN A string with and option like structure
        // WHEN parsed
        // THEN the Option token does not eat the newline
        let input = "--option\nempty item\n";
        let expected = TokenStream::from_pairs(&[
            (TK::Option, "--option"),
            (TK::NewLine, "\n"),
            (TK::Word, "empty"),
            (TK::Spaces, " "),
            (TK::Word, "item"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "abc\x07def\n";
        let expected = TokenStream::from_pairs(&[
            (TK::Word, "abc"),
            (TK::LiteralChar, "\x07"),
            (TK::Word, "def"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_converts_indents_to_relative_indents_and_dedents() {
        let input = "line_1\n    nested\n  dedented\n";
        let expected = TokenStream::from_pairs(&[
            (TK::Word, "line_1"),
            (TK::NewLine, "\n"),
            (TK::Indent, "    "),
            (TK::Word, "nested"),
            (TK::NewLine, "\n"),
            (TK::Dedent, "  "),
            (TK::Word, "dedented"),
            (TK::NewLine, "\n"),
            (TK::Dedent, "  "),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_emits_dedent_when_indented_block_returns_to_zero_indent() {
        let input = "line_1\n    nested\nplain\n";
        let expected = TokenStream::from_pairs(&[
            (TK::Word, "line_1"),
            (TK::NewLine, "\n"),
            (TK::Indent, "    "),
            (TK::Word, "nested"),
            (TK::NewLine, "\n"),
            (TK::Dedent, "    "),
            (TK::Word, "plain"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input), expected);
    }
}
