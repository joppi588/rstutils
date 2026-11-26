// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::inline_markup::parse_inline_token;

use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;

pub(crate) fn parse_paragraph(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    loop {
        let node = if let Some(node) = parse_inline_token(stream)? {
            node
        } else if stream.token_at_cursor().kind.is(TC::PARAGRAPH_END) {
            break;
        } else {
            let plain_text = parse_plain(stream);
            if stream.token_at_cursor().kind == TK::LiteralBlockMinimized {
                // TODO: Adds a colon, simplify this
                let text = plain_text
                    .borrow()
                    .attributes
                    .get_str("text")
                    .unwrap_or_default();

                plain_text.with_attr("text", format!("{text}:\n"));
            }
            plain_text
        };
        paragraph.push_child(node);
    }
    Ok(paragraph)
}

fn parse_plain(stream: &mut TokenStream) -> NodeRef {
    let kind = stream.token_at_cursor().kind;
    debug_assert!(
        !(kind.is(&TC::PARAGRAPH_END)
            || kind.is(&TC::INLINE_MARKUP)
            || kind.is(&TC::INLINE_REFERENCE)
            || kind == TK::InlineInternalTargetStart)
            || kind == TK::BackquoteStart
    );

    let mut text = String::new();

    loop {
        match stream.token_at_cursor().kind {
            kind if kind.is(&TC::PARAGRAPH_END)
                || kind.is(&TC::INLINE_MARKUP)
                || kind.is(&TC::INLINE_REFERENCE)
                || kind == TK::BackquoteStart
                || kind == TK::InlineInternalTargetStart =>
            {
                break;
            }
            _ => {
                text.push_str(&stream.consume().lexeme);
            }
        }
    }
    let sentence = AstNode::new_ref(NodeClass::PlainText);
    sentence.with_attr("text", text);
    sentence
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parsers::with_hanging_indent;

    /// GIVEN a paragraph consisting of a base line and an indented continuation line
    /// WHEN parse_paragraph is invoked with the expected hanging indent width
    /// THEN the paragraph has the hanging_indent attribute and contains both lines
    #[test]
    fn parse_paragraph_with_hanging_indent_two_lines() {
        let input = "First line\n   second line\n";
        let mut stream = tokenize(input);
        stream.consume(); //synthetic blank line

        let paragraph = with_hanging_indent(&mut stream, parse_paragraph)
            .expect("Paragraph parses successfully");
        let paragraph = paragraph.borrow();

        assert_eq!(
            paragraph.attributes.get_str("hanging_indent").as_deref(),
            Some("3")
        );

        let first = paragraph.children[0].borrow();
        assert_eq!(
            first.attributes.get_str("text").as_deref(),
            Some("First line\nsecond line\n")
        );
    }

    /// GIVEN a paragraph with a base line and two continuation lines at the same hanging indent
    /// WHEN parse_paragraph is invoked with the hanging indent width
    /// THEN the hanging_indent attribute is preserved and the continuation text is captured
    #[test]
    fn parse_paragraph_with_two_hanging_indent_lines() {
        let input = "First line\n   second line\n   third line\n";
        let mut stream = tokenize(input);
        stream.consume(); //synthetic blank line

        let paragraph = with_hanging_indent(&mut stream, parse_paragraph)
            .expect("Paragraph parses successfully");
        let paragraph = paragraph.borrow();

        assert_eq!(
            paragraph.attributes.get_str("hanging_indent").as_deref(),
            Some("3")
        );

        let plain = paragraph.children[0].borrow();
        assert_eq!(
            plain.attributes.get_str("text").as_deref(),
            Some("First line\nsecond line\nthird line\n")
        );
    }
}
