// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind as TK};
use crate::token_stream::{Position, TokenStream};

pub fn tokenize(input: &str) -> TokenStream {
    let input = format!("\n\n{input}\n"); // leading blank line
    let mut tokens = TokenStream::new(Vec::new());
    let mut last_token_kind = TK::BlankLine;
    let mut current_indent = 0;
    let mut position = Position::default();

    let mut index: usize = 1;
    while index < input.len() - 1 {
        let sub_str = &input[index - 1..];
        let (token_kind, lexeme, next_offset) = TK::match_token(sub_str)
            .unwrap_or_else(|| panic!("No token matched input: {sub_str:?}"));
        let new_token = Token::new(token_kind, lexeme);
        match (token_kind, last_token_kind) {
            (TK::BlankLine, _) => tokens.push(new_token, position),
            (TK::Indent, _) => {
                let new_indent = lexeme.len();
                if new_indent > current_indent {
                    let indent_token = Token::indent(new_indent - current_indent);
                    tokens.push(indent_token, position);
                } else if new_indent < current_indent {
                    let dedent_token = Token::dedent(current_indent - new_indent);
                    if last_token_kind == TK::BlankLine {
                        tokens.insert_at(tokens.len() - 1, dedent_token);
                    } else {
                        tokens.push(dedent_token, position);
                    }
                }
                current_indent = new_indent;
            }
            (_, TK::NewLine) => {
                // Not indented
                if current_indent > 0 {
                    let dedent_token = Token::dedent(current_indent);
                    tokens.push(dedent_token, position);
                }
                current_indent = 0;
                tokens.push(new_token, position);
            }
            (_, TK::BlankLine) => {
                // Not indented, insert dedent before blankline
                if current_indent > 0 {
                    let dedent_token = Token::dedent(current_indent);
                    tokens.insert_at(tokens.len() - 1, dedent_token);
                }
                current_indent = 0;
                tokens.push(new_token, position);
            }

            _ => tokens.push(new_token, position),
        }

        last_token_kind = token_kind;
        index += next_offset;
        position = position.advance(lexeme);
    }
    if current_indent > 0 {
        tokens.push(Token::dedent(current_indent), position);
    };
    tokens.push(Token::new(TK::EoF, ""), position);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token_stream::TokenStream;

    #[test]
    fn tokenize_spaces_words() {
        let input = "Hello World\n";
        let expected = TokenStream::from_pairs(&[
            (TK::BlankLine, "\n"),
            (TK::Word, "Hello"),
            (TK::Spaces, " "),
            (TK::Word, "World"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input).tokens(), expected.tokens());
    }

    #[test]
    fn tokenize_unicode_bullet_markers() {
        for marker in ["•", "‣", "⁃"] {
            let input = format!("{marker} item\n");
            let tokens = tokenize(&input).tokens();
            assert!(tokens
                .iter()
                .any(|token| { token.kind == TK::BulletListMarker && token.lexeme == marker }));
        }
    }

    #[test]
    fn tokenize_option_no_newline() {
        // GIVEN A string with and option like structure
        // WHEN parsed
        // THEN the Option token does not eat the newline
        let input = "--option\nempty item\n";
        let expected = TokenStream::from_pairs(&[
            (TK::BlankLine, "\n"),
            (TK::Option, "--option"),
            (TK::NewLine, "\n"),
            (TK::Word, "empty"),
            (TK::Spaces, " "),
            (TK::Word, "item"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input).tokens(), expected.tokens());
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "abc\x07def\n";
        let expected = TokenStream::from_pairs(&[
            (TK::BlankLine, "\n"),
            (TK::Word, "abc"),
            (TK::LiteralChar, "\x07"),
            (TK::Word, "def"),
            (TK::NewLine, "\n"),
            (TK::EoF, ""),
        ]);

        assert_eq!(tokenize(input).tokens(), expected.tokens());
    }

    #[test]
    fn tokenize_converts_indents_to_relative_indents_and_dedents() {
        let input = "line_1\n    nested\n  dedented\n";
        let expected = TokenStream::from_pairs(&[
            (TK::BlankLine, "\n"),
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

        assert_eq!(tokenize(input).tokens(), expected.tokens());
    }

    #[test]
    fn tokenize_emits_dedent_when_indented_block_returns_to_zero_indent() {
        let input = "line_1\n    nested\nplain\n";
        let expected = TokenStream::from_pairs(&[
            (TK::BlankLine, "\n"),
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

        assert_eq!(tokenize(input).tokens(), expected.tokens());
    }

    fn located(input: &str) -> Vec<(TK, usize, usize)> {
        let stream = tokenize(input);
        stream
            .tokens()
            .iter()
            .enumerate()
            .map(|(index, token)| {
                let position = stream.position_at(index);
                (token.kind, position.line, position.column)
            })
            .collect()
    }

    /// GIVEN a tab and a multibyte character in one line
    /// WHEN the source is tokenized
    /// THEN columns count characters, with a tab as one column
    #[test]
    fn tokenize_counts_columns_in_characters() {
        assert_eq!(
            located("A\t\u{e9}lan\n"),
            [
                (TK::BlankLine, 0, 1),
                (TK::Word, 1, 1),
                (TK::Spaces, 1, 2),
                (TK::Word, 1, 3),
                (TK::NewLine, 1, 7),
                (TK::EoF, 2, 1),
            ]
        );
    }

    /// GIVEN empty input
    /// WHEN the source is tokenized
    /// THEN EoF is at line 1, column 1
    #[test]
    fn tokenize_empty_input_positions_eof_at_start() {
        assert_eq!(located(""), [(TK::BlankLine, 0, 1), (TK::EoF, 1, 1)]);
    }

    /// GIVEN an indented line followed by an unindented one
    /// WHEN the source is tokenized
    /// THEN indent, dedent and following tokens carry their source positions
    #[test]
    fn tokenize_positions_indent_and_dedent() {
        assert_eq!(
            located("one\n    two\nthree\n"),
            [
                (TK::BlankLine, 0, 1),
                (TK::Word, 1, 1),
                (TK::NewLine, 1, 4),
                (TK::Indent, 2, 1),
                (TK::Word, 2, 5),
                (TK::NewLine, 2, 8),
                (TK::Dedent, 3, 1),
                (TK::Word, 3, 1),
                (TK::NewLine, 3, 6),
                (TK::EoF, 4, 1),
            ]
        );
    }

    /// GIVEN an indentation reduced to a smaller nonzero width
    /// WHEN the source is tokenized
    /// THEN the next word's column includes the consumed whitespace
    #[test]
    fn tokenize_counts_reduced_indentation_in_columns() {
        let tokens = located("root\n    nested\n  reduced\n");

        assert!(tokens.contains(&(TK::Word, 3, 3)));
    }
}
