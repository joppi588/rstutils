// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod parser_errors;
pub mod token;
pub mod token_slice;

use rstu_ast::{AstNode, NodeClass, NodeRef};

use crate::lexer::tokenize;
use crate::token::{Token, TokenCategory as TC, TokenKind as TK};
use parser_errors::{FindElementError, EXPECT_NEWLINE};
use token_slice::{find_next_kind, tokens_to_text};

// static DEDENT_GRACE: usize = 1;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, FindElementError> {
    let tokens = tokenize(input);
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut index: usize = 0;
    let mut current_node = doc.clone();

    while index < tokens.len() - 2 {
        // final two tokens are always Newline+Blankline
        let index_line_end = find_next_kind(&tokens, &[TK::NewLine], index)
            .expect("Token stream shall end with a newline.");
        match (tokens[index].kind, tokens[index_line_end + 1].kind) {
            (token1, token2)
                if (token1, token2) == (TK::Separator, TK::Indent)
                    || (token1, token2) == (TK::Separator, TK::Word)
                    || (token1, token2) == (TK::Word, TK::Separator) =>
            {
                let (section, next_start) =
                    try_match_section_header(&tokens, index, token1.is(&[TK::Separator]))?;
                AstNode::push_section_ref(&current_node, section.clone());
                current_node = section;
                index = next_start;
            }

            (TK::DoubleDot, _) => {
                let (directive, next_start) = try_parse_directive_like(&tokens, index)?;
                AstNode::push_body_element(&current_node, directive.clone());
                current_node = directive;
                index = next_start;
            }

            (TK::NewLine, TK::BlankLine)
            | (TK::BlankLine, _)
            | (TK::Indent, _)
            | (TK::Dedent, _) => index += 1,

            (kind, _) if kind.is(TC::INLINE_MARKER) || kind.is(TC::PLAIN) => {
                let (paragraph, next_start) = try_parse_paragraph(&tokens, index)?;
                AstNode::push_child(&current_node, paragraph.clone());
                index = next_start;
            }

            _ => panic!(
                "Unexpected token combination ({:?},{:?})",
                tokens[index].kind,
                tokens[index_line_end + 1].kind
            ),
        };
    }

    Ok(doc)
}

pub fn try_match_section_header(
    tokens: &[Token],
    start_at: usize,
    has_overline: bool,
) -> Result<(NodeRef, usize), FindElementError> {
    let title_start = start_at + 2 * usize::from(has_overline);
    let title_end = find_next_kind(tokens, &[TK::NewLine], title_start).map_err(|_| {
        FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        }
    })?;

    let closing_index = title_end + 1;
    if (closing_index >= tokens.len()) || (tokens[closing_index].kind != TK::Separator) {
        return Err(FindElementError::SectionTitleMissingClosingAfterOpening {
            opening_index: start_at,
        });
    }
    let closing_style = tokens[closing_index].lexeme.clone();

    if has_overline {
        let opening_style = tokens[start_at].lexeme.clone(); // TODO: single char + opening/closing length
        if tokens[start_at].lexeme != tokens[closing_index].lexeme {
            return Err(FindElementError::SectionTitleUnbalancedStyle {
                opening_index: start_at,
                opening_style,
                closing_style: closing_style.clone(),
            });
        }
    }

    let section = AstNode::new_ref(NodeClass::Section);
    AstNode::with_attr(&section, "section_marker", closing_style);

    let title = AstNode::new_ref(NodeClass::Title);
    AstNode::with_text(&title, tokens_to_text(&tokens[title_start..title_end]));
    AstNode::push_child(&section, title);

    Ok((section, closing_index + 2))
}

/// Parse directives, comments, citations, substitutions
fn try_parse_directive_like(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let index = find_next_kind(
        tokens,
        &[
            TK::NewLine,
            TK::DoubleColon,
            TK::FootnoteReferenceOpen,
            TK::HyperlinkReference,
            TK::SubstitutionReference,
        ],
        start_at,
    )
    .expect(EXPECT_NEWLINE);
    let (directive, new_index) = match &tokens[index].kind {
        TK::NewLine => try_parse_comment(tokens, start_at, index)?,
        TK::DoubleColon => try_parse_directive(tokens, start_at, index)?,
        _ => panic!("Not implemented directive-like structure."),
    };
    Ok((directive, new_index))
}

fn try_parse_comment(
    tokens: &[Token],
    start_at: usize,
    first_line_end: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let mut index = first_line_end;
    if tokens[index + 1].kind == TK::Indent {
        index = find_next_kind(tokens, &[TK::Dedent], index + 1)
            .expect("There is always a final dedent.");
    }

    let comment = AstNode::new_ref(NodeClass::Comment);
    let comment_tokens = token_slice::tokens_without_kinds(
        &tokens[start_at + 2..index + 1],
        &[TK::Indent, TK::Dedent],
    ); // skip '.. '
    AstNode::with_text(&comment, token_slice::tokens_to_text(&comment_tokens));
    Ok((comment, index + 1))
}

fn try_parse_directive(
    tokens: &[Token],
    start_at: usize,
    directive_colon_index: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let first_line_end =
        find_next_kind(tokens, &[TK::NewLine], directive_colon_index).expect(EXPECT_NEWLINE);

    let directive_type = tokens_to_text(&tokens[start_at + 1..directive_colon_index])
        .trim()
        .to_string();
    let directive_text = tokens_to_text(&tokens[directive_colon_index + 1..first_line_end]);

    let directive = AstNode::new_ref(NodeClass::Directive);
    AstNode::with_attr(&directive, "directive_type", directive_type);
    if !directive_text.is_empty() {
        AstNode::with_text(&directive, directive_text);
    }

    let index = first_line_end + 1;
    if index >= tokens.len() || tokens[index].kind != TK::Indent {
        return Ok((directive, index));
    }

    let indentation = tokens[index].lexeme.clone();

    let indented_block = AstNode::new_ref(NodeClass::Block);
    AstNode::with_attr(&indented_block, "indentation", indentation);
    let (paragraph, index) = try_parse_paragraph(&tokens, index + 1)?;
    AstNode::push_child(&indented_block, paragraph);
    AstNode::push_child(&directive, indented_block);

    Ok((directive, index))
}

fn try_parse_paragraph(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let paragraph_end = find_next_kind(
        tokens,
        &[TK::BlankLine, TK::Indent, TK::Separator, TK::Dedent],
        start_at,
    )
    .expect("Paragraph must end somewhere.");
    let paragraph = AstNode::new_ref(NodeClass::Paragraph);
    let mut index = start_at;
    while index < paragraph_end {
        let (node, new_index) = match tokens[index].kind {
            kind if kind.is(TC::INLINE_MARKER) => try_parse_inline(&tokens, index)?,
            kind if kind.is(TC::PLAIN) || kind == TK::Punctuation => {
                try_parse_plain(&tokens, index)?
            }
            kind if kind.is(TC::CONTROL) => {
                index += 1;
                continue;
            }
            _ => {
                return Err(FindElementError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", tokens[index].kind),
                });
            }
        };
        index = new_index;
        AstNode::push_child(&paragraph, node);
    }
    Ok((paragraph, paragraph_end + 1))
}

fn try_parse_inline(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let inline_final = match tokens[start_at].kind {
        TK::Strong => find_next_kind(tokens, &[TK::Strong], start_at + 1)
            .map_err(|_| FindElementError::StrongMissingClosing { start_at: start_at })?,
        _ => {
            return Err(FindElementError::UnexpectedToken {
                expected: "Inline".to_owned(),
                found: format!("{:?}", tokens[start_at].kind),
            });
        }
    };
    let strong = AstNode::new_ref(NodeClass::InlineMarkup);
    AstNode::with_attr(&strong, "markup", "strong");
    AstNode::with_text(&strong, tokens_to_text(&tokens[start_at + 1..inline_final]));
    Ok((strong, inline_final + 1))
}

fn try_parse_plain(
    tokens: &[Token],
    start_at: usize,
) -> Result<(NodeRef, usize), FindElementError> {
    let plain_tokens = find_next_kind(
        tokens,
        &[
            TK::Strong,
            TK::BlankLine,
            TK::DoubleDot,
            TK::Indent,
            TK::Dedent,
        ], // TODO implement kinds_except
        start_at,
    )
    .map_err(|_| FindElementError::InvalidPlainText { start_at: start_at })?;
    let sentence = AstNode::new_ref(NodeClass::PlainText);
    AstNode::with_attr(
        &sentence,
        "text",
        tokens_to_text(&tokens[start_at..plain_tokens]),
    );
    Ok((sentence, plain_tokens))
}
