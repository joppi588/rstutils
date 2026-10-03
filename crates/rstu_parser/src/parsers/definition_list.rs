// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block;
use super::paragraph::{parse_inline, parse_inline_token};
use crate::parser_errors::ParserError;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

pub(crate) fn is_definition_list_item(stream: &TokenStream) -> bool {
    return stream.token_at_cursor().kind.nested_is(TC::PARAGRAPH)
        && stream.token_at_nextline().kind == TK::Indent;
}

pub(crate) fn parse_definition_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::DefinitionList);

    while is_definition_list_item(stream) {
        let item = parse_definition_list_item(stream)?;
        list.push_child(item);
        if stream.token_at_cursor().kind == TK::BlankLine {
            list.push_blank_lines(stream.consume().len());
        }
    }
    if stream.token_at_cursor().kind == TK::EoF
        || stream.token_peek_relative(-1).kind == TK::BlankLine
        || (stream.token_at_cursor().kind == TK::Dedent
            && stream.token_peek_relative(1).kind == TK::BlankLine)
    {
        return Ok(list);
    }
    Err(ParserError::ListMissingBlankLineError {})
}

fn parse_definition_list_item(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let item = AstNode::new_ref(NodeClass::DefinitionListItem);
    let mut node_class = NodeClass::Term;
    loop {
        let node = AstNode::new_ref(node_class);
        parse_term_or_classifier(stream, &node)?;
        item.push_child(node);
        node_class = NodeClass::Classifier;

        match stream.token_at_cursor().kind {
            TK::ClassifierSeparator => {
                stream.consume();
            }
            _ => {
                stream.consume_newline();
                break;
            }
        }
    }

    let definition = AstNode::new_ref(NodeClass::Definition);
    let block = parse_block(stream)?;

    definition.push_child(block);
    item.push_child(definition);
    Ok(item)
}

fn is_term_text(kind: TK) -> bool {
    kind.is(TC::PLAIN) || matches!(kind, TK::BulletListMarker | TK::Field)
}

fn parse_term_or_classifier(stream: &mut TokenStream, node: &NodeRef) -> Result<(), ParserError> {
    loop {
        let kind = stream.token_at_cursor().kind;
        match kind {
            TK::NewLine | TK::ClassifierSeparator | TK::EoF => break,
            kind if kind.is(TC::INLINE_MARKER) => {
                let inline = parse_inline(stream)?;
                let markup = inline.borrow().attributes.get_str("markup");
                let text = inline.borrow().attributes.get_str("text");
                match markup.as_deref() {
                    Some("inline_literal") => {
                        inline.with_attr("markup", "literal");
                        node.push_child(inline);
                    }
                    Some("hyperlink_reference") => {
                        let text = text.unwrap_or_default();
                        let reference = AstNode::new_ref(NodeClass::Reference);
                        reference.with_attr("refname", text.clone());
                        let plain_text = AstNode::new_ref(NodeClass::PlainText);
                        plain_text.with_attr("text", text);
                        reference.push_child(plain_text);
                        node.push_child(reference);
                    }
                    _ => node.push_child(inline),
                }
            }
            kind if kind.is(TC::INLINE_TOKEN) => node.push_child(parse_inline_token(stream)?),
            kind if is_term_text(kind) => {
                let mut text = String::new();
                while {
                    let kind = stream.token_at_cursor().kind;
                    kind != TK::NewLine && is_term_text(kind)
                } {
                    text.push_str(&stream.consume().lexeme);
                }
                if !text.is_empty() {
                    let plain_text = AstNode::new_ref(NodeClass::PlainText);
                    plain_text.with_attr("text", text);
                    node.push_child(plain_text);
                }
            }
            _ => {
                return Err(ParserError::UnexpectedToken {
                    expected: "Inline/plain".to_owned(),
                    found: format!("{:?}", kind),
                    index: stream.cursor(),
                });
            }
        }
    }
    Ok(())
}
