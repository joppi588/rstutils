// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{Block, Comment, Directive, Document, Heading, IndentedBlock, Sentence};
use crate::lexer::tokenize;
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub token_index: usize,
}

impl ParseError {
    fn new(message: impl Into<String>, token_index: usize) -> Self {
        Self {
            message: message.into(),
            token_index,
        }
    }
}

// TODO: move this into the Parser class
pub fn parse(input: &str) -> Result<Document, ParseError> {
    let tokens = tokenize(input);
    let mut parser = Parser { tokens, pos: 0 };
    parser.parse_document()
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}


// We have the following cases:
// Blank line: 
// - starts a new block (eventually indented if the current one is indented)
// - leaves a block if unindented
// - ignore if it follows a blank line
// see pest rules


impl Parser {
    // TODO: Check if other parser loop over the rules the same way.
    // REFACTOR: use a list of functions, then a loop or a macro
    fn parse_document(&mut self) -> Result<Document, ParseError> {
        let mut blocks = Vec::new();

        self.skip_blank_lines();
        while !self.is_eof() {
            if let Some(block) = self.try_parse_heading()? {
                blocks.push(Block::Heading(block));
            } else if let Some(block) = self.try_parse_comment()? {
                blocks.push(Block::Comment(block));
            } else if let Some(block) = self.try_parse_directive()? {
                blocks.push(Block::Directive(block));
            } else {
                return Err(ParseError::new("unexpected token sequence", self.pos));
            }

            self.skip_blank_lines();
        }

        Ok(Document::new(blocks))
    }

    fn try_parse_heading(&mut self) -> Result<Option<Heading>, ParseError> {
        let checkpoint = self.pos;

        if self.peek_kind(TokenKind::Word).is_none() {
            return Ok(None);
        }

        let title = match self.parse_sentence_until(TokenKind::NewLine)? {
            Some(sentence) => sentence,
            None => return Ok(None),
        };

        if !self.consume_kind(TokenKind::NewLine) {
            self.pos = checkpoint;
            return Ok(None);
        }

        let Some(underline) = self.consume_lexeme(TokenKind::HeadingUnderline) else {
            self.pos = checkpoint;
            return Ok(None);
        };

        self.consume_kind(TokenKind::NewLine);

        Ok(Some(Heading { title, underline }))
    }

    fn try_parse_comment(&mut self) -> Result<Option<Comment>, ParseError> {
        let checkpoint = self.pos;

        if !self.consume_kind(TokenKind::DoubleDot) {
            return Ok(None);
        }

        if !self.consume_kind(TokenKind::Spaces) {
            self.pos = checkpoint;
            return Ok(None);
        }

        let text = match self.parse_sentence_until(TokenKind::NewLine) {
            Ok(Some(sentence)) => sentence,
            Ok(None) | Err(_) => {
                self.pos = checkpoint;
                return Ok(None);
            }
        };

        self.expect_kind(TokenKind::NewLine, "expected newline after comment")?;

        Ok(Some(Comment { text }))
    }

    fn try_parse_directive(&mut self) -> Result<Option<Directive>, ParseError> {
        let checkpoint = self.pos;

        if !self.consume_kind(TokenKind::DoubleDot) {
            return Ok(None);
        }

        if !self.consume_kind(TokenKind::Spaces) {
            self.pos = checkpoint;
            return Ok(None);
        }

        let Some(mut name) = self.consume_directive_name() else {
            self.pos = checkpoint;
            return Ok(None);
        };

        if !self.consume_directive_marker(&mut name) {
            self.pos = checkpoint;
            return Ok(None);
        }

        let argument = self.parse_sentence_until(TokenKind::NewLine)?;
        self.expect_kind(TokenKind::NewLine, "expected newline after directive header")?;

        let body = self.parse_indented_block()?;

        Ok(Some(Directive {
            name,
            argument,
            body,
        }))
    }

    fn parse_indented_block(&mut self) -> Result<IndentedBlock, ParseError> {
        let Some(indent) = self.consume_lexeme(TokenKind::Indent) else {
            return Err(ParseError::new(
                "expected indented block after directive",
                self.pos,
            ));
        };

        let mut lines = Vec::new();

        let first_line = self
            .parse_sentence_until(TokenKind::NewLine)?
            .ok_or_else(|| ParseError::new("expected sentence in indented block", self.pos))?;
        self.expect_kind(TokenKind::NewLine, "expected newline after indented sentence")?;
        lines.push(first_line);

        while let Some(next_indent) = self.peek_kind(TokenKind::Indent) {
            if next_indent.lexeme != indent {
                break;
            }

            self.pos += 1;

            let line = self
                .parse_sentence_until(TokenKind::NewLine)?
                .ok_or_else(|| ParseError::new("expected sentence in indented block", self.pos))?;
            self.expect_kind(TokenKind::NewLine, "expected newline after indented sentence")?;
            lines.push(line);
        }

        Ok(IndentedBlock { indent, lines })
    }

    fn parse_sentence_until(&mut self, stop: TokenKind) -> Result<Option<Sentence>, ParseError> {
        let mut parts: Vec<String> = Vec::new();
        let mut saw_word = false;

        while let Some(token) = self.tokens.get(self.pos) {
            if token.kind == stop {
                break;
            }

            match token.kind {
                TokenKind::Word => {
                    saw_word = true;
                    parts.push(token.lexeme.clone());
                    self.pos += 1;
                }
                TokenKind::Spaces | TokenKind::LiteralString | TokenKind::Bold => {
                    parts.push(token.lexeme.clone());
                    self.pos += 1;
                }
                _ => return Err(ParseError::new("unexpected token in sentence", self.pos)),
            }
        }

        if !saw_word {
            return Ok(None);
        }

        Ok(Some(Sentence::new(parts.concat())))
    }

    fn consume_directive_marker(&mut self, directive_name: &mut String) -> bool {
        if self.consume_kind(TokenKind::DoubleColon) {
            return true;
        }

        let mut colons = 0;

        if directive_name.ends_with(':') {
            directive_name.pop();
            colons += 1;
        }

        while let Some(token) = self.tokens.get(self.pos) {
            if token.kind == TokenKind::LiteralString && token.lexeme == ":" {
                self.pos += 1;
                colons += 1;
                if colons == 2 {
                    return true;
                }
                continue;
            }
            break;
        }

        false
    }

    fn consume_directive_name(&mut self) -> Option<String> {
        if let Some(name) = self.consume_lexeme(TokenKind::Word) {
            return Some(name);
        }

        let token = self.peek_kind(TokenKind::LiteralString)?;
        let is_name = token
            .lexeme
            .chars()
            .all(|chr| chr.is_ascii_alphanumeric() || chr == '_' || chr == ':');

        if !is_name || token.lexeme.is_empty() {
            return None;
        }

        self.consume_lexeme(TokenKind::LiteralString)
    }

    fn skip_blank_lines(&mut self) {
        while self.consume_kind(TokenKind::BlankLine) || self.consume_kind(TokenKind::NewLine) {}
    }

    fn peek_kind(&self, kind: TokenKind) -> Option<&Token> {
        self.tokens.get(self.pos).filter(|token| token.kind == kind)
    }

    fn consume_kind(&mut self, kind: TokenKind) -> bool {
        if self.peek_kind(kind).is_some() {
            self.pos += 1;
            return true;
        }

        false
    }

    fn consume_lexeme(&mut self, kind: TokenKind) -> Option<String> {
        if let Some(token) = self.peek_kind(kind) {
            let lexeme = token.lexeme.clone();
            self.pos += 1;
            return Some(lexeme);
        }

        None
    }

    fn expect_kind(&mut self, kind: TokenKind, message: &'static str) -> Result<(), ParseError> {
        if self.consume_kind(kind) {
            return Ok(());
        }

        Err(ParseError::new(message, self.pos))
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len() || self.peek_kind(TokenKind::LiteralString).is_some_and(|token| token.lexeme.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::parse;
    use rstu_ast::Block;

    #[test]
    fn parses_heading_comment_and_directive() {
        let input = "Heading\n=======\n\n.. comment one\n\n.. note:: body\n   one line.\n   two line.\n";

        let document = parse(input).expect("expected valid parse");
        assert_eq!(document.blocks.len(), 3);

        assert!(matches!(document.blocks[0], Block::Heading(_)));
        assert!(matches!(document.blocks[1], Block::Comment(_)));
        assert!(matches!(document.blocks[2], Block::Directive(_)));
    }

    #[test]
    fn rejects_directive_without_indented_block() {
        let input = ".. note:: body\nnot indented\n";

        let result = parse(input);
        assert!(result.is_err());
    }
}
