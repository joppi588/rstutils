// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::block::parse_block;
use super::list_enum_helpers::{
    enumerator_parts, enumerator_type, enumerator_value, resolve_enumerator_type,
};
use crate::parser_errors::{ParserError, EXPECT_NEWLINE};
use crate::token::{Token, TokenKind as TK};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::debug_assert_matches;

fn prepare_item_block(stream: &mut TokenStream, dedent_len: usize) -> Result<(), ParserError> {
    // List item cases
    // 1. [Optional Blankline],  Indent -> Hanging indent block
    // 2. [Optional Blankline], list marker -> Single line item
    // 3. Non-indented paragraph etc -> Error
    let next_line = stream.find_next_kind(&[TK::NewLine]).expect(EXPECT_NEWLINE) + 1;

    let indent_ahead_index = match stream.token_at(next_line).kind {
        TK::Indent => Some(next_line),
        TK::BlankLine if stream.token_at(next_line + 1).kind == TK::Indent => Some(next_line + 1),
        _ => None,
    };

    match indent_ahead_index {
        Some(indent_index) => {
            let indent_token = stream.take_at(indent_index);
            if indent_token.len() <= dedent_len {
                stream.insert_before_cursor(indent_token);
            } else {
                // If the next line is indented beyond the marker/field/...,
                // we assume that it represents two subsequent indents.
                stream.insert_before_cursor(Token::indent(dedent_len));
                stream.insert_at(
                    indent_index + 1,
                    Token::indent(indent_token.len() - dedent_len),
                );
            }
        }
        None => match stream.token_at(next_line).kind {
            // list end
            TK::Field
            | TK::BulletListMarker
            | TK::EnumeratedListMarker
            | TK::EoF
            | TK::Dedent
            | TK::BlankLine => {
                stream.insert_before_cursor(Token::indent(dedent_len));
                stream.insert_at(next_line + 1, Token::dedent(dedent_len));
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

    if stream.token_at_cursor().kind == TK::Spaces {
        dedent_len += stream.consume().len();
    }

    prepare_item_block(stream, dedent_len)?;
    let block = parse_block(stream).map_err(|_| ParserError::ListEndError {})?;
    item.push_child(block);
    list.push_child(item);
    if stream.token_at_cursor().kind == TK::BlankLine {
        let blank_token = stream.consume();
        list.push_blank_lines(blank_token.len());
    }
    Ok(())
}

pub(crate) fn parse_bullet_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::BulletListMarker);

    let list = AstNode::new_ref(NodeClass::BulletList);
    let mut marker: Option<String> = None;

    while stream.token_at_cursor().kind == TK::BulletListMarker {
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

pub(crate) fn parse_enumerated_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::EnumeratedListMarker);

    let first_marker = stream.token_at_cursor().lexeme.to_owned();
    let (prefix, first_value, suffix) = enumerator_parts(&first_marker);
    let enumtype = resolve_enumerator_type(enumerator_type(first_value)?, None, None);
    let list = AstNode::new_ref(NodeClass::EnumeratedList);
    list.with_attr("enumtype", format!("{enumtype:?}").to_lowercase())
        .with_attr("prefix", prefix)
        .with_attr("suffix", suffix);

    let mut next_number = 1;
    let mut previous_value: Option<String> = None;
    while stream.token_at_cursor().kind == TK::EnumeratedListMarker {
        let marker = stream.token_at_cursor().lexeme.to_owned();
        let (item_prefix, value, item_suffix) = enumerator_parts(&marker);
        let item_type = if value == "#" {
            enumtype
        } else {
            resolve_enumerator_type(
                enumerator_type(value)?,
                Some(enumtype),
                previous_value.as_deref(),
            )
        };
        if item_prefix != prefix || item_suffix != suffix || item_type != enumtype {
            break;
        }
        stream.consume(); // Consume the marker used above
        previous_value = Some(value.to_owned());
        let item = AstNode::new_ref(NodeClass::EnumeratedListItem);
        item.with_attr("raw_value", value);
        let number = match value {
            "#" => next_number,
            _ => enumerator_value(value, enumtype)?,
        };
        item.with_attr("number", number);
        next_number = number + 1;
        finish_list_item(stream, &list, item, marker.len())?;
    }

    Ok(list)
}

pub(crate) fn parse_field_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::Field);

    let list = AstNode::new_ref(NodeClass::FieldList);

    while stream.token_at_cursor().kind == TK::Field {
        let item = AstNode::new_ref(NodeClass::FieldListItem);
        let field_token = stream.consume();
        let field_name = field_token
            .lexeme
            .strip_prefix(':')
            .and_then(|name| name.strip_suffix(':'))
            .unwrap()
            .to_owned();
        item.with_attr("fieldname", field_name);

        let dedent_len = field_token.len();
        finish_list_item(stream, &list, item, dedent_len)?;
    }

    Ok(list)
}

#[cfg(test)]
mod tests {
    use crate::parse;
    use rstu_ast::NodeClass;

    fn list_types(input: &str) -> Vec<String> {
        parse(input)
            .unwrap()
            .borrow()
            .children
            .iter()
            .filter(|child| child.borrow().class == NodeClass::EnumeratedList)
            .map(|child| child.borrow().attributes.get_str("enumtype").unwrap())
            .collect()
    }

    #[test]
    fn ambiguous_i_starts_a_roman_list() {
        // GIVEN a list whose first marker is the ambiguous I
        // WHEN the list is parsed
        // THEN the list style is upper Roman
        assert_eq!(list_types("I. first\nII. second\n"), ["upperroman"]);
    }

    #[test]
    fn ambiguous_c_starts_an_alpha_list() {
        // GIVEN a list whose first marker is the ambiguous C
        // WHEN the list is parsed
        // THEN the list style is upper alpha
        assert_eq!(list_types("C. first\nD. second\n"), ["upperalpha"]);
    }

    #[test]
    fn ambiguous_markers_use_the_existing_list_style() {
        // GIVEN alpha and Roman lists containing ambiguous markers
        // WHEN the lists are parsed
        // THEN I follows alpha and C follows Roman
        assert_eq!(list_types("H. first\nI. second\n"), ["upperalpha"]);
        assert_eq!(list_types("I. first\nC. second\n"), ["upperroman"]);
    }

    #[test]
    fn ambiguous_i_starts_a_roman_list_unless_preceded_by_h() {
        // GIVEN alpha lists whose next marker is I or i
        // WHEN that marker is preceded by something other than H or h
        // THEN it starts a Roman list
        assert_eq!(
            list_types("F. first\nI. second\nII. third\n"),
            ["upperalpha", "upperroman"]
        );
        assert_eq!(
            list_types("f. first\ni. second\nii. third\n"),
            ["loweralpha", "lowerroman"]
        );
        assert_eq!(list_types("H. first\nI. second\n"), ["upperalpha"]);
        assert_eq!(list_types("h. first\ni. second\n"), ["loweralpha"]);
    }
}
