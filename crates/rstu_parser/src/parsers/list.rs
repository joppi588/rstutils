// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block;
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

fn prepare_item_block(stream: &mut TokenStream, dedent_len: usize) -> Result<(), ParserError> {
    // List item cases
    // 1. [Optional Blankline],  Indent -> Hanging indent block
    // 2. [Optional Blankline], list marker -> Single line item
    // 3. Non-indented paragraph etc -> Error
    let next_line = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE) + 1;

    let indent_ahead_index = match stream.kind_at(next_line) {
        TK::Indent => Some(next_line),
        TK::BlankLine if stream.kind_at(next_line + 1) == TK::Indent => Some(next_line + 1),
        _ => None,
    };

    match indent_ahead_index {
        Some(indent_index) => {
            let indent_token = stream.take_at(indent_index);
            let cursor = stream.cursor();
            if indent_token.lexeme.len() <= dedent_len {
                stream.insert_at(cursor, indent_token);
            } else {
                // If the next line is indented beyond the marker/field/...,
                // we assume that it represents two subsequent indents.
                stream.insert_at(cursor, Token::new(TK::Indent, " ".repeat(dedent_len)));
                stream.insert_at(
                    indent_index + 1,
                    Token::new(
                        TK::Indent,
                        " ".repeat(indent_token.lexeme.len() - dedent_len),
                    ),
                );
            }
            stream.set_cursor(cursor);
        }
        None => match stream.kind_at(next_line) {
            TK::Field
            | TK::BulletListMarker
            | TK::EnumeratedListMarker
            | TK::EoF
            | TK::Dedent
            | TK::BlankLine => {
                stream.insert_at(
                    stream.cursor(),
                    Token::new(TK::Indent, " ".repeat(dedent_len)),
                );
                stream.set_cursor(stream.cursor() - 1);
                stream.insert_at(
                    next_line + 1,
                    Token::new(TK::Dedent, " ".repeat(dedent_len)),
                );
            }
            _ => return Err(ParserError::ListEndError {}),
        },
    }
    Ok(())
}

// TODO: Make this the main function and do the preparation depending on the list marker.
fn finish_list_item(
    stream: &mut TokenStream,
    list: &NodeRef,
    item: NodeRef,
    dedent_len: usize,
) -> Result<(), ParserError> {
    let mut dedent_len = dedent_len;

    if stream.kind_at_cursor() == TK::Spaces {
        dedent_len += stream.consume().lexeme.len();
    }

    prepare_item_block(stream, dedent_len)?;
    let block = parse_block(stream).map_err(|_| ParserError::ListEndError {})?;
    item.push_child(block);
    list.push_child(item);
    if stream.kind_at_cursor() == TK::BlankLine {
        let blank_token = stream.consume();
        list.push_blank_lines(blank_token.lexeme.len());
    }
    Ok(())
}

pub(crate) fn parse_bullet_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut marker: Option<String> = None;

    while stream.kind_at_cursor() == TK::BulletListMarker {
        let item = AstNode::new_ref(NodeClass::BulletListItem);
        let marker_token = stream.consume();
        item.with_attr("marker", marker_token.lexeme.clone());
        if let Some(existing_marker) = &marker {
            if existing_marker != &marker_token.lexeme {
                return Err(ParserError::ListStyleError {
                    marker: existing_marker.clone(),
                    conflicting_marker: marker_token.lexeme,
                });
            }
        } else {
            marker = Some(marker_token.lexeme);
        }

        finish_list_item(stream, &list, item, 1)?;
    }

    Ok(list)
}

fn alphabetic_value(value: &str) -> Option<usize> {
    if value.is_empty()
        || !value
            .chars()
            .all(|character| character.is_ascii_alphabetic())
    {
        return None;
    }

    Some(value.chars().fold(0, |total, character| {
        total * 26 + (character.to_ascii_uppercase() as usize - 'A' as usize + 1)
    }))
}

fn roman_value(value: &str) -> Option<usize> {
    if value.is_empty() {
        return None;
    }

    let mut total = 0;
    let mut previous = 0;
    for character in value.chars().rev() {
        let current = match character.to_ascii_uppercase() {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => return None,
        };
        if current < previous {
            total -= current;
        } else {
            total += current;
            previous = current;
        }
    }
    Some(total)
}

fn enumerator_parts(marker: &str) -> (&str, &str, &str) {
    let (prefix, value, suffix) = if marker.starts_with('(') && marker.ends_with(')') {
        ("(", &marker[1..marker.len() - 1], ")")
    } else {
        let split_at = marker.len().saturating_sub(1);
        ("", &marker[..split_at], &marker[split_at..])
    };
    (prefix, value, suffix)
}

fn enumerator_value(value: &str, enumtype: &str) -> Option<usize> {
    match enumtype {
        "arabic" => value.parse().ok(),
        "upperalpha" | "loweralpha" => alphabetic_value(value),
        "upperroman" | "lowerroman" => roman_value(value),
        _ => None,
    }
}

fn enumerator_type(value: &str) -> Option<&'static str> {
    if value.chars().all(|character| character.is_ascii_digit()) {
        Some("arabic")
    } else if roman_value(value).is_some()
        && value.chars().all(|character| {
            matches!(
                character.to_ascii_uppercase(),
                'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'
            )
        })
    {
        if value
            .chars()
            .all(|character| character.is_ascii_uppercase())
        {
            Some("upperroman")
        } else {
            Some("lowerroman")
        }
    } else if value
        .chars()
        .all(|character| character.is_ascii_uppercase())
    {
        Some("upperalpha")
    } else if value
        .chars()
        .all(|character| character.is_ascii_lowercase())
    {
        Some("loweralpha")
    } else {
        None
    }
}

pub(crate) fn parse_enumerated_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let first_marker = stream.token_at(stream.cursor()).lexeme.trim().to_owned();
    let (prefix, first_value, suffix) = enumerator_parts(&first_marker);
    let enumtype = enumerator_type(first_value).ok_or(ParserError::ListEndError {})?;
    let list = AstNode::new_ref(NodeClass::EnumeratedList);
    list.with_attr("enumtype", enumtype)
        .with_attr("prefix", prefix)
        .with_attr("suffix", suffix);

    let mut next_number = 1;
    while stream.kind_at_cursor() == TK::EnumeratedListMarker {
        let marker = stream.consume().lexeme.trim().to_owned();
        let (item_prefix, value, item_suffix) = enumerator_parts(&marker);
        if item_prefix != prefix
            || item_suffix != suffix
            || (value != "#" && enumerator_type(value) != Some(enumtype))
        {
            break;
        }
        let number = if value == "#" {
            next_number
        } else {
            enumerator_value(value, enumtype).ok_or(ParserError::ListEndError {})?
        };
        next_number = number + 1;

        let item = AstNode::new_ref(NodeClass::EnumeratedListItem);
        item.with_attr("number", number);
        finish_list_item(stream, &list, item, 1)?;
    }

    Ok(list)
}

pub(crate) fn parse_field_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    let list = AstNode::new_ref(NodeClass::FieldList);

    while stream.kind_at_cursor() == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_token = stream.consume();
        let field_name = field_token
            .lexeme
            .strip_prefix(':')
            .and_then(|name| name.strip_suffix(':'))
            .unwrap()
            .to_owned();
        item.with_attr("fieldname", field_name);

        let dedent_len = field_token.lexeme.len();
        finish_list_item(stream, &list, item, dedent_len)?;
    }

    Ok(list)
}
