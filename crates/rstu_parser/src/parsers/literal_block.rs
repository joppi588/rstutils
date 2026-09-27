// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;

#[derive(Debug)]
pub enum LiteralBlockType {
    Expanded,
    PartiallyMinimized,
    FullyMinimized,
}

pub(crate) fn parse_literal_block(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert!(stream.kind_at_cursor().is(&[
        TK::LiteralBlock,
        TK::LiteralBlockMinimized,
        TK::LiteralBlockPartiallyMinimized
    ]));

    let block = AstNode::new_ref(NodeClass::LiteralBlock);

    let literal_block_token = stream.consume();
    let literal_block_type = match literal_block_token.kind {
        TK::LiteralBlockMinimized => LiteralBlockType::FullyMinimized,
        TK::LiteralBlock => LiteralBlockType::Expanded,
        TK::LiteralBlockPartiallyMinimized => LiteralBlockType::PartiallyMinimized,
        _ => unreachable!(),
    };
    block.with_attr("type", format!("{literal_block_type:?}"));

    if stream.consume().kind != TK::BlankLine {
        return Err(ParserError::LiteralBlockError {
            message: "literal block requires a blank line".to_owned(),
        });
    }
    if stream.kind_at_cursor() == TK::Indent {
        set_indented_content(stream, &block)?;
    } else if stream.kind_at_cursor() != TK::EoF {
        append_quoted_content(stream, &block)?;
    } else {
        return Err(ParserError::LiteralBlockError {
            message: "literal block expected after literal marker".to_owned(),
        });
    }

    Ok(block)
}

fn set_indented_content(stream: &mut TokenStream, block: &NodeRef) -> Result<(), ParserError> {
    let mut rel_indent = stream.consume().lexeme.len();
    block.with_attr("indent", rel_indent);

    let mut text = String::new();
    loop {
        match stream.kind_at_cursor() {
            TK::EoF => {
                break;
            }
            TK::Indent => {
                let indent = stream.consume();
                rel_indent += indent.lexeme.len();
                text.push_str(&indent.lexeme.to_string());
            }
            TK::Dedent => {
                let dedent_token = stream.token_at(stream.cursor());
                let dedent = dedent_token.lexeme.len();
                if dedent > rel_indent {
                    stream.update_at_cursor(" ".repeat(dedent - rel_indent));
                    break;
                } else if dedent == rel_indent {
                    stream.set_cursor(stream.cursor() + 1);
                    break;
                } else {
                    rel_indent -= dedent;
                }
            }
            _ => text.push_str(&stream.consume().lexeme),
        }
    }

    if text.is_empty() {
        return Err(ParserError::LiteralBlockError {
            message: "literal block expected after literal marker".to_owned(),
        });
    }
    if text.ends_with("\n\n") {
        text.pop();
    }
    block.with_attr("text", text);
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
