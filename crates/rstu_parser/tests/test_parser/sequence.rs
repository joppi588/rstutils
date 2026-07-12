// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::Block;
use rstu_parser::parse;

#[test]
fn parses_heading_comment_and_directive_sequence() {
    let input = "Heading\n=======\n\n.. comment one\n\n.. note:: body\n   one line.\n";

    let document = parse(input).expect("expected valid sequence");
    assert_eq!(document.blocks.len(), 3);
    assert!(matches!(document.blocks[0], Block::Heading(_)));
    assert!(matches!(document.blocks[1], Block::Comment(_)));
    assert!(matches!(document.blocks[2], Block::Directive(_)));
}
