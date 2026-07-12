// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::Block;
use rstu_parser::parse;

#[test]
fn parses_directive_with_indented_block_into_ast() {
    let input = ".. note:: short arg\n   first line.\n   second line.\n";

    let document = parse(input).expect("expected valid directive");
    assert_eq!(document.blocks.len(), 1);

    let Block::Directive(directive) = &document.blocks[0] else {
        panic!("expected directive block");
    };

    assert_eq!(directive.name, "note");
    assert_eq!(directive.argument.as_ref().map(|s| s.text.as_str()), Some(" short arg"));
    assert_eq!(directive.body.indent, "   ");
    assert_eq!(directive.body.lines.len(), 2);
    assert_eq!(directive.body.lines[0].text, "first line.");
    assert_eq!(directive.body.lines[1].text, "second line.");
}

#[test]
fn rejects_directive_without_indented_block() {
    let input = ".. note:: arg\nnot indented\n";

    let result = parse(input);
    assert!(result.is_err());
}
