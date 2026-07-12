// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::doctree::Block;
use rstu_parser::parse;

#[test]
fn parses_heading_into_doctree() {
    let input = "Title Line\n==========\n\n";

    let document = parse(input).expect("expected valid heading");
    assert_eq!(document.blocks.len(), 1);

    let Block::Heading(heading) = &document.blocks[0] else {
        panic!("expected heading block");
    };

    assert_eq!(heading.title.text, "Title Line");
    assert_eq!(heading.underline, "==========");
}

#[test]
fn parses_comment_into_doctree() {
    let input = ".. comment text.\n";

    let document = parse(input).expect("expected valid comment");
    assert_eq!(document.blocks.len(), 1);

    let Block::Comment(comment) = &document.blocks[0] else {
        panic!("expected comment block");
    };

    assert_eq!(comment.text.text, "comment text.");
}

#[test]
fn parses_directive_with_indented_block_into_doctree() {
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

#[test]
fn parses_heading_comment_and_directive_sequence() {
    let input = "Heading\n=======\n\n.. comment one\n\n.. note:: body\n   one line.\n";

    let document = parse(input).expect("expected valid sequence");
    assert_eq!(document.blocks.len(), 3);
    assert!(matches!(document.blocks[0], Block::Heading(_)));
    assert!(matches!(document.blocks[1], Block::Comment(_)));
    assert!(matches!(document.blocks[2], Block::Directive(_)));
}
