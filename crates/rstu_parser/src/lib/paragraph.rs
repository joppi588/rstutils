// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use crate::token_slice::{find_next_kind, tokens_to_text};

pub(crate) fn try_parse_paragraph(
    tokens: &[Token],
    start_at: usize,
    stop_before: Option<usize>,
    skip_index: Option<usize>,
) -> Result<(NodeRef, usize), ParserError> {
    let paragraph_end = stop_before.unwrap_or(
        find_next_kind(
            tokens,
            &[TK::BlankLine, TK::Indent, TK::Separator, TK::Dedent],
            start_at,
            skip_index,
        )
        .expect("Paragraph must end somewhere."),
    );
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    let mut index = start_at;
    while index < paragraph_end {
        if skip_index == Some(index) {
            index += 1;
            continue;
        }

        let (node, new_index) = match tokens[index].kind {
            kind if kind.is(TC::INLINE_MARKER) => try_parse_inline(&tokens, index)?,
            kind if kind.is(TC::INLINE_TOKEN) => try_parse_inline_token(&tokens, index)?,
            //TODO: Concatenate TC::PLAIN and tokens to a new list
            kind if kind.is(TC::PLAIN) || kind == TK::BulletListMarker || kind == TK::NewLine => {
                try_parse_plain(&tokens, index, paragraph_end, skip_index)?
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

pub(crate) fn try_parse_inline_token(
    tokens: &[Token],
    at: usize,
) -> Result<(NodeRef, usize), ParserError> {
    let node = AstNode::new_ref(NodeClass::Reference);
    let kind = tokens[at].kind;
    let lexeme = &tokens[at].lexeme;
    match kind {
        TK::FootnoteReference => {
            node.with_text(&lexeme[1..lexeme.len() - 2])
                .with_attr("type", "footnote");
        }
        TK::SubstitutionReference => {
            node.with_text(&lexeme[1..lexeme.len() - 1])
                .with_attr("type", "sub");
        }
        TK::SimpleHyperlinkReference => {
            node.with_text(&lexeme[0..lexeme.len() - 1])
                .with_attr("type", "simple_ref");
        }
        TK::SimpleAnonymousHyperLinkReference => {
            node.with_text(&lexeme[0..lexeme.len() - 2])
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

pub(crate) fn try_parse_inline(
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

    let inline_final =
        find_next_kind(tokens, end_kind_candidates, start_at + 1, None).map_err(|_| {
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
        .with_text(tokens_to_text(&tokens[start_at + 1..inline_final]));
    Ok((inline, inline_final + 1))
}

fn try_parse_plain(
    tokens: &[Token],
    start_at: usize,
    stop_before: usize,
    skip_index: Option<usize>,
) -> Result<(NodeRef, usize), ParserError> {
    let mut index = start_at;
    let mut text = String::new();
    while index < stop_before {
        if skip_index == Some(index) {
            index += 1;
            continue;
        }
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
    sentence.with_text(text);
    Ok((sentence, index))
}

#[cfg(test)]
mod tests {
    use super::try_parse_paragraph;
    use crate::token::{Token, TokenKind as TK};

    #[test]
    fn try_parse_paragraph_skips_the_requested_index() {
        let tokens = vec![
            Token::new(TK::Word, "hello"),
            Token::new(TK::Word, "world"),
            Token::new(TK::Word, "again"),
            Token::new(TK::BlankLine, "\n"),
        ];

        let (paragraph, next_index) = try_parse_paragraph(&tokens, 0, None, Some(1))
            .expect("paragraph parsing should succeed");

        assert_eq!(next_index, 3);
        assert_eq!(
            paragraph.borrow().children[0].borrow().text,
            Some("helloagain".into())
        );
    }
}
