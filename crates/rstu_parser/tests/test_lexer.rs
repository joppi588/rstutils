// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::lexer::tokenize;
use rstu_parser::token::{Token, TokenKind};
use std::fs;
use std::path::Path;

#[test]
fn tokenize_ok_mixed_lorem_ipsum_file() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data/ok_mixed_lorem_ipsum.rst");
    let contents = fs::read_to_string(path).expect("failed to read mixed lorem ipsum test file");

    let tokens: Vec<Token> = tokenize(&contents);
    let kinds: Vec<(TokenKind,&str)> = tokens.iter().map(|token| token.as_tuple()).collect();
    let expected_kinds = vec![
       (TokenKind::Word, "Lorem"), 
       (TokenKind::Spaces, " "), 
       (TokenKind::Word, "Ipsum"), 
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "Heading"),
       (TokenKind::NewLine, "\n"),
       (TokenKind::HeadingUnderline, "==================="),
       (TokenKind::NewLine, "\n"),
       (TokenKind::BlankLine, "\n"),
       (TokenKind::DoubleDot, ".."),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "note"),
       (TokenKind::DoubleColon, "::"),
       (TokenKind::NewLine, "\n"),
       (TokenKind::Indent, "   "),
       (TokenKind::Word, "Lorem"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "ipsum"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "dolor"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "sit"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "amet"),
       (TokenKind::LiteralString, ","),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "consectetur"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "adipiscing"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "elit"),
       (TokenKind::LiteralString, "."),
       (TokenKind::NewLine, "\n"),
       (TokenKind::Indent, "   "),
       (TokenKind::Word, "Vivamus"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "lacinia"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "odio"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "vitae"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "vestibulum"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "vestibulum"),
       (TokenKind::LiteralString, "."),
       (TokenKind::NewLine, "\n"),
       (TokenKind::Indent, "   "),
       (TokenKind::Word, "Cras"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "venenatis"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "euismod"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "malesuada"),
       (TokenKind::LiteralString, "."),
       (TokenKind::NewLine, "\n"),
       (TokenKind::BlankLine, "\n"),
       (TokenKind::DoubleDot, ".."),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "comment"),
       (TokenKind::NewLine, "\n"),
       (TokenKind::BlankLine, "\n"),
       (TokenKind::Bold, "**"),
       (TokenKind::Word, "end"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "of"),
       (TokenKind::Spaces, " "),
       (TokenKind::Word, "file"),
       (TokenKind::Bold, "**"),
       (TokenKind::NewLine, "\n"),
       (TokenKind::LiteralString,"")
    ];

    assert_eq!(kinds, expected_kinds);
}
