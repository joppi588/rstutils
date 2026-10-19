// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;

pub(crate) fn parse_paragraph_hanging_indent(
    stream: &mut TokenStream,
) -> Result<NodeRef, ParserError> {
    // TODO: Move this upstream?
    let index_next_line: usize = stream.find_end_of_line() + 1;
    let indent = match stream.token_at(index_next_line).kind {
        TK::Indent => stream.take_at(index_next_line).len(),
        _ => 0,
    };
    let paragraph = parse_paragraph(stream)?;
    if indent > 0 {
        paragraph.with_attr("hanging_indent", indent);

        let dedent = stream.token_at_cursor();
        // TODO: Can we reuse this logic somewhere?
        if dedent.kind != TK::Dedent {
            return Err(ParserError::UnexpectedToken {
                expected: "Dedent".to_string(),
                found: format!("{:?}", dedent.kind),
                position: stream.position_at_cursor(),
            });
        } else if dedent.len() > indent {
            stream.update_at_cursor(" ".repeat(dedent.len() - indent));
        } else if dedent.len() < indent {
            return Err(ParserError::DedentToSmall {
                expected: dedent.len(),
                found: indent,
                position: stream.position_at_cursor(),
            });
        } else {
            stream.consume();
        }
    }
    Ok(paragraph)
}

pub(crate) fn parse_paragraph(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    loop {
        let kind = stream.token_at_cursor().kind;
        let node = match kind {
            kind if kind.is(TC::INLINE_MARKER) => parse_inline(stream)?,
            kind if kind.is(TC::INLINE_TOKEN) => parse_inline_token(stream)?,
            kind if kind.is(TC::PARAGRAPH_END) => break,
            _ => {
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
            }
        };
        paragraph.push_child(node);
    }
    Ok(paragraph)
}

pub(crate) fn parse_inline_token(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_TOKEN));

    let node = AstNode::new_ref(NodeClass::Reference);
    let position = stream.position_at_cursor();
    let token = stream.consume();
    let kind = token.kind;
    let lexeme = &token.lexeme;
    match kind {
        TK::FootnoteReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 2])
                .with_attr("type", "footnote");
        }
        TK::SubstitutionReference => {
            node.with_attr("text", &lexeme[1..lexeme.len() - 1])
                .with_attr("type", "sub");
        }
        TK::SimpleHyperlinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 1])
                .with_attr("type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_attr("text", &lexeme[0..lexeme.len() - 2])
                .with_attr("type", "simple_anonymous_ref");
        }

        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Reference token".to_owned(),
                found: format!("{:?}", kind),
                position,
            });
        }
    };
    Ok(node)
}

pub(crate) fn parse_inline(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.token_at_cursor().kind.is(TC::INLINE_MARKER));

    let position = stream.position_at_cursor();
    let kind = stream.consume().kind;
    let (markup, end_kind_candidates): (&str, &[TK]) = match kind {
        TK::StrongStart => ("strong", &[TK::StrongEnd]),
        TK::EmphasisStart => ("emphasis", &[TK::EmphasisEnd]),
        TK::InlineLiteralStart => ("inline_literal", &[TK::InlineLiteralEnd]),
        TK::InlineInternalTargetStart => ("inline_internal_target", &[TK::BackquoteEnd]),
        TK::BackquoteStart => (
            "interpreted_or_hyperlink",
            &[TK::BackquoteEnd, TK::HyperlinkReferenceEnd],
        ),
        _ => {
            return Err(ParserError::UnexpectedToken {
                expected: "Inline start token".to_owned(),
                found: format!("{:?}", kind),
                position,
            });
        }
    };

    let text = stream.consume_text_until(&[end_kind_candidates, TC::PARAGRAPH_END].concat());
    if !stream.token_at_cursor().kind.is(end_kind_candidates) {
        return Err(ParserError::InlineMissingClosing {
            markup: markup.to_owned(),
            position,
        });
    }

    let effective_markup = match (kind, stream.consume().kind) {
        (TK::BackquoteStart, TK::HyperlinkReferenceEnd) => "hyperlink_reference",
        (TK::BackquoteStart, TK::BackquoteEnd) => "interpreted_text",
        _ => markup,
    };

    let inline = AstNode::new_ref(NodeClass::InlineMarkup);
    inline
        .with_attr("markup", effective_markup)
        .with_attr("text", text);
    Ok(inline)
}

fn parse_plain(stream: &mut TokenStream) -> NodeRef {
    debug_assert!(
        stream.token_at_cursor().kind.is(TC::PLAIN)
            || matches!(
                stream.token_at_cursor().kind,
                TK::BulletListMarker | TK::Option
            )
    );

    let mut text = String::new();

    loop {
        match stream.token_at_cursor().kind {
            kind if kind.is(&TC::PLAIN)
                || matches!(
                    kind,
                    TK::BulletListMarker | TK::ClassifierSeparator | TK::Option
                ) =>
            {
                text.push_str(&stream.consume().lexeme);
            }
            _ => break,
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

    /// GIVEN a paragraph consisting of a base line and an indented continuation line
    /// WHEN parse_paragraph is invoked with the expected hanging indent width
    /// THEN the paragraph has the hanging_indent attribute and contains both lines
    #[test]
    fn parse_paragraph_with_hanging_indent_two_lines() {
        let input = "First line\n   second line\n";
        let mut stream = tokenize(input);
        stream.consume(); //synthetic blank line

        let paragraph =
            parse_paragraph_hanging_indent(&mut stream).expect("Paragraph parses successfully");
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

        let paragraph =
            parse_paragraph_hanging_indent(&mut stream).expect("Paragraph parses successfully");
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
