// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::Block;
use rstu_parser::parse;

#[test]
fn parses_heading_into_ast() {
    let input = "Title Line\n==========\n\n";

    let document = parse(input).expect("expected valid heading");
    assert_eq!(document.blocks.len(), 1);

    let Block::Heading(heading) = &document.blocks[0] else {
        panic!("expected heading block");
    };

    assert_eq!(heading.title.text, "Title Line");
    assert_eq!(heading.underline, "==========");
}
