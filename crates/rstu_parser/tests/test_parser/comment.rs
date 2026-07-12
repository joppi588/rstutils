// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::Block;
use rstu_parser::parse;

#[test]
fn parses_comment_into_ast() {
    let input = ".. comment text.\n";

    let document = parse(input).expect("expected valid comment");
    assert_eq!(document.blocks.len(), 1);

    let Block::Comment(comment) = &document.blocks[0] else {
        panic!("expected comment block");
    };

    assert_eq!(comment.text.text, "comment text.");
}
