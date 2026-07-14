// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenizeMode {
    Stage1Raw,
    Stage2Composed,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    tokenize_with_mode(input, TokenizeMode::Stage2Composed)
}

pub fn tokenize_with_mode(input: &str, mode: TokenizeMode) -> Vec<Token> {
    let stage1 = tokenize_stage1_raw(input);
    match mode {
        TokenizeMode::Stage1Raw => stage1,
        TokenizeMode::Stage2Composed => compose_stage2(stage1),
    }
}

fn tokenize_stage1_raw(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let input = format!("\n{input}\n");
    let mut remaining = input.as_str();

    while remaining.len() > 1 {
        let mut best_match: Option<(TokenKind, usize, usize)> = None;
        let mut literal_match: Option<(usize, usize)> = None;

        // Optimizations
        // Don't use capture groups, as we know that the context is 1char
        // use find_at
        for kind in TokenKind::ALL {
            let token_match = match kind.regex().captures_iter(&remaining).find_map(|captures| {
                captures
                    .get(1)
                    .filter(|token_match| token_match.start() > 0)
            }) {
                Some(token_match) => token_match,
                None => continue,
            };

            let candidate = (kind, token_match.start(), token_match.end());

            // Not needed?
            if kind == TokenKind::LiteralString {
                literal_match = Some((candidate.1, candidate.2));
                continue;
            }

            // Optimize: if candidate is found, continue immediately
            let replace = match best_match {
                Some((_, best_start, _)) => candidate.1 < best_start,
                None => true,
            };

            if replace {
                best_match = Some(candidate);
            }
        }

        let Some((kind, start, end)) = best_match else {
            let Some((start, end)) = literal_match else {
                break;
            };

            tokens.push(Token::new(TokenKind::LiteralString, &remaining[start..end]));
            break;
        };

        if start > 1 {
            let literal = &remaining[1..start];
            if !literal.is_empty() {
                tokens.push(Token::new(TokenKind::LiteralString, literal));
            }
            remaining = &remaining[(start - 1)..];
            continue;
        }

        tokens.push(Token::new(kind, &remaining[start..end]));

        // Optimization: just update the start pointer
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
        remaining = &remaining[keep_context_from..];
    }

    tokens
}

fn compose_stage2(tokens: Vec<Token>) -> Vec<Token> {
    let mut composed = Vec::with_capacity(tokens.len());
    let mut i = 0;

    while i < tokens.len() {
        if i + 3 < tokens.len()
            && tokens[i].kind == TokenKind::DoubleDot
            && tokens[i + 1].kind == TokenKind::Spaces
            && tokens[i + 2].kind == TokenKind::Word
            && tokens[i + 3].kind == TokenKind::DoubleColon
        {
            let directive = [
                tokens[i].lexeme.as_str(),
                tokens[i + 1].lexeme.as_str(),
                tokens[i + 2].lexeme.as_str(),
                tokens[i + 3].lexeme.as_str(),
            ]
            .concat();
            composed.push(Token::new(TokenKind::Directive, directive));
            i += 4;
            continue;
        }

        composed.push(tokens[i].clone());
        i += 1;
    }

    composed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_spaces_words() {
        let input = "Hello World";
        let expected = vec![
            Token::new(TokenKind::Word, "Hello"),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::Word, "World"),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_stage1_keeps_directive_parts_split() {
        let input = ".. note::";
        let expected = vec![
            Token::new(TokenKind::DoubleDot, ".."),
            Token::new(TokenKind::Spaces, " "),
            Token::new(TokenKind::Word, "note"),
            Token::new(TokenKind::DoubleColon, "::"),
        ];

        assert_eq!(tokenize_with_mode(input, TokenizeMode::Stage1Raw), expected);
    }

    #[test]
    fn tokenize_stage2_composes_directive() {
        let input = ".. note::";
        let expected = vec![Token::new(TokenKind::Directive, ".. note::")];

        assert_eq!(
            tokenize_with_mode(input, TokenizeMode::Stage2Composed),
            expected
        );
    }

    // Note: This test fails (bug)
    #[test]
    fn tokenize_treats_unmatched_input_as_literal_string() {
        let input = "*%*%*";
        let expected = vec![Token::new(TokenKind::LiteralString, "*%*%*")];

        assert_eq!(tokenize(input), expected);
    }
}
