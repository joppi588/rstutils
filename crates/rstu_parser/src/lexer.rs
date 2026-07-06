// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

// Lexer development on hold

pub fn tokenize(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (idx, chr) in input.char_indices() {
        if chr == '\n' {
            tokens.push(input[start..idx].to_string());
            tokens.push("\n".to_string());
            start = idx + chr.len_utf8();
        }
    }

    tokens.push(input[start..].to_string());
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_preserves_newlines() {
        let input = "Hello\nWorld\n";
        let expected = vec![
            "Hello".to_string(),
            "\n".to_string(),
            "World".to_string(),
            "\n".to_string(),
            "".to_string(),
        ];

        assert_eq!(tokenize(input), expected);
    }

    #[test]
    fn tokenize_empty_string_returns_empty_line() {
        assert_eq!(tokenize(""), vec!["".to_string()]);
    }
}
