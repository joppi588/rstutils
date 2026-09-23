// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;

pub(crate) fn parse_literal_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    if stream
        .token_at(stream.cursor().saturating_sub(1))
        .lexeme
        .ends_with('\\')
    {
        return Err(ParserError::LiteralBlockError {
            message: "escaped literal marker".to_owned(),
        });
    }

    let marker = stream.consume();
    debug_assert_eq!(marker.kind, TK::DoubleColon);

    let block = AstNode::new_ref(NodeClass::LiteralBlock);
    block.with_attr("xml:space", "preserve");

    if stream.kind_at_cursor() == TK::Spaces {
        stream.consume();
    }

    if stream.kind_at_cursor() == TK::NewLine {
        stream.consume();
    }

    if stream.kind_at_cursor() == TK::BlankLine {
        stream.consume();
    } else if stream.kind_at_cursor() == TK::Indent {
        return Err(ParserError::LiteralBlockError {
            message: "literal block requires a blank line".to_owned(),
        });
    }

    if stream.kind_at_cursor() == TK::Indent {
        let indent = stream.consume().lexeme.len();
        block.with_attr("indent", indent);
        append_indented_content(stream, &block)?;
    } else if stream.kind_at_cursor() != TK::EoF {
        append_quoted_content(stream, &block)?;
    } else {
        return Err(ParserError::LiteralBlockError {
            message: "literal block expected after literal marker".to_owned(),
        });
    }

    Ok(block)
}

fn append_indented_content(stream: &mut TokenStream, block: &NodeRef) -> Result<(), ParserError> {
    let mut text = String::new();
    while stream.kind_at_cursor() != TK::EoF {
        match stream.kind_at_cursor() {
            TK::BlankLine | TK::Dedent => {
                break;
            }
            _ => text.push_str(&stream.consume().lexeme),
        }
    }

    if text.is_empty() {
        return Err(ParserError::LiteralBlockError {
            message: "literal block expected after literal marker".to_owned(),
        });
    }
    if !text.ends_with('\n') {
        text.push('\n');
    }

    let content = AstNode::new_ref(NodeClass::PlainText);
    content.with_attr("text", text);
    block.push_child(content);
    Ok(())
}

fn append_quoted_content(stream: &mut TokenStream, block: &NodeRef) -> Result<(), ParserError> {
    let mut text = String::new();
    let mut at_line_start = true;
    while !matches!(stream.kind_at_cursor(), TK::BlankLine | TK::EoF) {
        if at_line_start {
            if stream.kind_at_cursor() == TK::Indent {
                return Err(ParserError::LiteralBlockError {
                    message: "unexpected indentation in quoted literal block".to_owned(),
                });
            }
            if stream.token_at(stream.cursor()).lexeme != ">" {
                return Err(ParserError::LiteralBlockError {
                    message: "inconsistent quoted literal block".to_owned(),
                });
            }
        }
        let token = stream.consume();
        at_line_start = token.kind == TK::NewLine;
        text.push_str(&token.lexeme);
    }

    if text.is_empty() {
        return Err(ParserError::LiteralBlockError {
            message: "literal block expected after literal marker".to_owned(),
        });
    }

    let content = AstNode::new_ref(NodeClass::PlainText);
    content.with_attr("text", text);
    block.push_child(content);
    Ok(())
}
