// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

const NON_LITERAL_TOKEN_KINDS: [TokenKind; 8] = [
    TokenKind::HeadingUnderline,
    TokenKind::Indent,
    TokenKind::DoubleColon,
    TokenKind::DoubleDot,
    TokenKind::TableHorizontal,
    TokenKind::BlankLine,
    TokenKind::NewLine,
    TokenKind::Word,
];

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut remaining = format!("\n{input}");

    while remaining.len() > 1 {
        let mut best_match: Option<(TokenKind, usize, usize)> = None;

        for kind in NON_LITERAL_TOKEN_KINDS {
            let token_match = match kind
                .regex()
                .captures_iter(&remaining)
                .find_map(|captures| {
                    captures
                        .get(1)
                        .filter(|token_match| token_match.start() > 0)
                })
            {
                Some(token_match) => token_match,
                None => continue,
            };

            let candidate = (kind, token_match.start(), token_match.end());
            let replace = match best_match {
                Some((_, best_start, _)) => candidate.1 < best_start,
                None => true,
            };

            if replace {
                best_match = Some(candidate);
            }
        }

        let Some((kind, start, end)) = best_match else {
            tokens.push(Token::new(TokenKind::LiteralString, &remaining[1..]));
            break;
        };

        if start > 1 {
            let literal = &remaining[1..start];
            if !literal.is_empty() {
                tokens.push(Token::new(TokenKind::LiteralString, literal));
            }
            remaining = remaining[(start - 1)..].to_string();
            continue;
        }

        tokens.push(Token::new(kind, &remaining[start..end]));

        let consume_until = if end > start {
            end
        } else {
            1 + remaining[1..]
                .chars()
                .next()
                .map(|chr| chr.len_utf8())
                .unwrap_or(0)
        };
        let keep_context_from = consume_until.saturating_sub(1);
        remaining = remaining[keep_context_from..].to_string();
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_emits_literal_when_next_match_is_not_at_second_character() {
        let input = "Hello World";
        let expected = vec![
            Token::new(TokenKind::Word, "Hello"),
            Token::new(TokenKind::LiteralString, " World"),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "***";
        let expected = vec![Token::new(TokenKind::LiteralString, "***")];

        assert_eq!(tokenize(input), expected);
    }
}
