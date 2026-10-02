// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block;
use super::paragraph::parse_inline_children;
use crate::lexer::tokenize;
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
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
        let blank_line = stream.consume();
        if !matches!(blank_line.kind, TK::EoF | TK::BlankLine) {
            return Err(ParserError::ListEndError {});
        }
        list.push_blank_lines(blank_line.len());
    }

    Ok(list)
}

fn parse_definition_list_item(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let item = AstNode::new_ref(NodeClass::DefinitionListItem);
    let line_text = stream
        .consume_text_until(&[TK::NewLine])
        .expect(EXPECT_NEWLINE);
    let segments = split_term_and_classifiers(&line_text);

    let term = AstNode::new_ref(NodeClass::Term);
    parse_line_segment(&segments[0], &term)?;
    item.push_child(term);
    for segment in segments.iter().skip(1) {
        let classifier = AstNode::new_ref(NodeClass::Classifier);
        parse_line_segment(segment, &classifier)?;
        item.push_child(classifier);
    }
    stream.consume_newline();

    let definition = AstNode::new_ref(NodeClass::Definition);
    let block = parse_block(stream).map_err(|_| ParserError::ListEndError {})?;

    definition.push_child(block);
    item.push_child(definition);
    Ok(item)
}

fn split_term_and_classifiers(line: &str) -> Vec<String> {
    let line = line.trim_end_matches('\n');
    let mut delimiters = Vec::new();
    let mut index = 0;
    let mut in_inline = false;
    while index < line.len() {
        let remaining = &line[index..];
        if remaining.starts_with('`') {
            in_inline = !in_inline;
            index += if remaining.starts_with("``") { 2 } else { 1 };
        } else if !in_inline && remaining.starts_with(" : ") {
            let escaped = line[..index].ends_with('\\');
            if !escaped {
                delimiters.push(index);
            }
            index += 3;
        } else {
            index += remaining.chars().next().unwrap().len_utf8();
        }
    }

    let mut segments = Vec::with_capacity(delimiters.len() + 1);
    let mut start = 0;
    for delimiter in delimiters {
        segments.push(line[start..delimiter].trim().replace("\\:", ":"));
        start = delimiter + 3;
    }
    segments.push(line[start..].trim().replace("\\:", ":"));
    segments
}

fn parse_line_segment(text: &str, node: &NodeRef) -> Result<(), ParserError> {
    let mut stream = tokenize(&format!("{text}\n"));
    parse_inline_children(&mut stream, node)?;

    let children = std::mem::take(&mut node.borrow_mut().children);
    for child in children {
        let (class, markup, text) = {
            let borrowed = child.borrow();
            (
                borrowed.class,
                borrowed.attributes.get_str("markup"),
                borrowed.attributes.get_str("text"),
            )
        };
        if class == NodeClass::InlineMarkup && markup.as_deref() == Some("inline_literal") {
            child.with_attr("markup", "literal");
        }
        if class == NodeClass::InlineMarkup && markup.as_deref() == Some("hyperlink_reference") {
            let text = text.unwrap_or_default();
            let reference = AstNode::new_ref(NodeClass::Reference);
            reference.with_attr("refname", text.clone());
            let plain_text = AstNode::new_ref(NodeClass::PlainText);
            plain_text.with_attr("text", text);
            reference.push_child(plain_text);
            node.push_child(reference);
        } else if class == NodeClass::PlainText
            && text.as_deref() == Some("\n")
            && node.borrow().children.last().is_some_and(|last| {
                matches!(
                    last.borrow().class,
                    NodeClass::InlineMarkup | NodeClass::Reference
                )
            })
        {
            continue;
        } else {
            node.push_child(child);
        }
    }

    Ok(())
}
