// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use crate::token_slice::{find_next_kind, tokens_to_text};

pub(crate) fn parse_paragraph(
    tokens: &[Token],
    start_at: usize,
    stop_before: Option<usize>,
) -> Result<(NodeRef, usize), ParserError> {
    let paragraph_end = stop_before.unwrap_or(
        find_next_kind(
            tokens,
            &[
                TK::BlankLine,
                TK::Indent,
                TK::Separator,
                TK::Dedent,
                TK::EoF,
            ],
            start_at,
        )
        .expect("Paragraph must end somewhere."),
    );
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    let mut index = start_at;
    while index < paragraph_end {
        let (node, new_index) = match tokens[index].kind {
            kind if kind.is(TC::INLINE_MARKER) => parse_inline(&tokens, index)?,
            kind if kind.is(TC::INLINE_TOKEN) => parse_inline_token(&tokens, index)?,
            //TODO: Concatenate TC::PLAIN and tokens to a new list
            kind if kind.is(TC::PLAIN) || kind == TK::BulletListMarker || kind == TK::NewLine => {
                parse_plain(&tokens, index, paragraph_end)?
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", tokens[index].kind),
                    index: index,
                });
            }
        };
        index = new_index;
        paragraph.push_child(node);
    }
    Ok((paragraph, index))
}

/// Parse a paragraph that continues after a hanging indent token: the indent is simply skipped.
pub(crate) fn parse_paragraph_with_hanging_indent(
    tokens: &[Token],
    start_at: usize,
    indent_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let (paragraph, _) = parse_paragraph(tokens, start_at, Some(indent_at))?;
    let (continuation, new_index) = parse_paragraph(tokens, indent_at + 1, None)?;
    for child in std::mem::take(&mut continuation.borrow_mut().children) {
        paragraph.push_child(child);
    }
    Ok((paragraph, new_index))
}

pub(crate) fn parse_inline_token(
    tokens: &[Token],
    at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let node = AstNode::new_ref(NodeClass::Reference);
    let kind = tokens[at].kind;
    let lexeme = &tokens[at].lexeme;
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
                index: at,
            });
        }
    };
    Ok((node, at + 1))
}

pub(crate) fn parse_inline(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let kind = tokens[start_at].kind;
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
                index: start_at,
            });
        }
    };

    let inline_final = find_next_kind(tokens, end_kind_candidates, start_at + 1).map_err(|_| {
        ParserError::InlineMissingClosing {
            markup: markup.to_owned(),
            start_at,
        }
    })?;

    let effective_markup = match (kind, tokens[inline_final].kind) {
        (TK::BackquoteStart, TK::HyperlinkReferenceEnd) => "hyperlink_reference",
        (TK::BackquoteStart, TK::BackquoteEnd) => "interpreted_text",
        _ => markup,
    };

    let inline = AstNode::new_ref(NodeClass::InlineMarkup);
    inline
        .with_attr("markup", effective_markup)
        .with_attr("text", tokens_to_text(&tokens[start_at + 1..inline_final]));
    Ok((inline, inline_final + 1))
}

fn parse_plain(
    tokens: &[Token],
    start_at: usize,
    stop_before: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let mut index = start_at;
    let mut text = String::new();
    while index < stop_before {
        // TODO: Use TC::PLAIN
        if !tokens[index].is(&[
            TK::Word,
            TK::Spaces,
            TK::Punctuation,
            TK::NewLine,
            TK::BulletListMarker,
        ]) {
            break;
        }

        text.push_str(&tokens[index].lexeme);
        index += 1;
    }

    let sentence = AstNode::new_ref(NodeClass::PlainText);
    sentence.with_attr("text", text);
    Ok((sentence, index))
}

#[cfg(test)]
mod tests {
    use super::{parse_paragraph, parse_paragraph_with_hanging_indent};
    use crate::token::{Token, TokenKind as TK};

    #[test]
    fn parse_paragraph_stops_before_the_requested_index() {
        let tokens = vec![
            Token::new(TK::Word, "hello"),
            Token::new(TK::Word, "world"),
            Token::new(TK::BlankLine, "\n"),
        ];

        let (paragraph, next_index) =
            parse_paragraph(&tokens, 0, Some(1)).expect("paragraph parsing should succeed");

        assert_eq!(next_index, 1);
        assert_eq!(
            paragraph.borrow().children[0]
                .borrow()
                .attributes
                .get("text"),
            Some(&"hello".into())
        );
    }

    #[test]
    fn parse_paragraph_with_hanging_indent_skips_the_indent_token() {
        let tokens = vec![
            Token::new(TK::Word, "hello"),
            Token::new(TK::Indent, "  "),
            Token::new(TK::Word, "again"),
            Token::new(TK::BlankLine, "\n"),
        ];

        let (paragraph, next_index) = parse_paragraph_with_hanging_indent(&tokens, 0, 1)
            .expect("paragraph parsing should succeed");

        assert_eq!(next_index, 3);
        let children = &paragraph.borrow().children;
        assert_eq!(children.len(), 2);
        assert_eq!(
            children[0].borrow().attributes.get("text"),
            Some(&"hello".into())
        );
        assert_eq!(
            children[1].borrow().attributes.get("text"),
            Some(&"again".into())
        );
    }
}
