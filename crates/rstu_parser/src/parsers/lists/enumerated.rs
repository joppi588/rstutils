// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{consume_trailing_blank_lines, parse_item_body};
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::debug_assert_matches;

#[derive(Debug, Clone, Copy, PartialEq)]
enum EnumType {
    Arabic,
    Upperalpha,
    Loweralpha,
    Upperroman,
    Lowerroman,
    UpperAmbiguousI,
    LowerAmbiguousI,
    UpperAmbiguousC,
    LowerAmbiguousC,
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
        stream.consume();
        previous_value = Some(value.to_owned());
        let item = AstNode::new_ref(NodeClass::EnumeratedListItem);
        item.with_attr("raw_value", value);
        let number = match value {
            "#" => next_number,
            _ => enumerator_value(value, enumtype)?,
        };
        item.with_attr("number", number);
        next_number = number + 1;
        item.push_child(parse_item_body(stream, marker.len())?);
        list.push_child(item);
        consume_trailing_blank_lines(stream, &list);
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

fn enumerator_value(value: &str, enumtype: EnumType) -> Result<usize, ParserError> {
    let converted_value = match enumtype {
        EnumType::Arabic => value.parse().ok(),
        EnumType::Upperalpha | EnumType::Loweralpha => alphabetic_value(value),
        EnumType::Upperroman | EnumType::Lowerroman => roman_value(value),
        EnumType::UpperAmbiguousI
        | EnumType::LowerAmbiguousI
        | EnumType::UpperAmbiguousC
        | EnumType::LowerAmbiguousC => None,
    };
    converted_value.ok_or_else(|| ParserError::ListMarkerError {
        marker: value.to_string(),
    })
}

fn enumerator_type(value: &str) -> Result<EnumType, ParserError> {
    match value {
        value if value.chars().all(|character| character.is_ascii_digit()) => Ok(EnumType::Arabic),
        "I" => Ok(EnumType::UpperAmbiguousI),
        "i" => Ok(EnumType::LowerAmbiguousI),
        "C" => Ok(EnumType::UpperAmbiguousC),
        "c" => Ok(EnumType::LowerAmbiguousC),
        "#" => Ok(EnumType::Arabic),
        value
            if value.chars().count() > 1
                && roman_value(value).is_some()
                && value
                    .chars()
                    .all(|character| character.is_ascii_uppercase()) =>
        {
            Ok(EnumType::Upperroman)
        }
        value
            if value.chars().count() > 1
                && roman_value(value).is_some()
                && value
                    .chars()
                    .all(|character| character.is_ascii_lowercase()) =>
        {
            Ok(EnumType::Lowerroman)
        }
        value
            if value
                .chars()
                .all(|character| character.is_ascii_uppercase()) =>
        {
            Ok(EnumType::Upperalpha)
        }
        value
            if value
                .chars()
                .all(|character| character.is_ascii_lowercase()) =>
        {
            Ok(EnumType::Loweralpha)
        }
        value => Err(ParserError::ListMarkerError {
            marker: value.to_string(),
        }),
    }
}

fn resolve_enumerator_type(
    marker_type: EnumType,
    list_type: Option<EnumType>,
    previous_value: Option<&str>,
) -> EnumType {
    match marker_type {
        EnumType::UpperAmbiguousI if previous_value != Some("H") => EnumType::Upperroman,
        EnumType::LowerAmbiguousI if previous_value != Some("h") => EnumType::Lowerroman,
        _ => match (marker_type, list_type) {
            (
                EnumType::UpperAmbiguousI | EnumType::UpperAmbiguousC,
                Some(EnumType::Upperroman | EnumType::Lowerroman),
            ) => EnumType::Upperroman,
            (
                EnumType::LowerAmbiguousI | EnumType::LowerAmbiguousC,
                Some(EnumType::Upperroman | EnumType::Lowerroman),
            ) => EnumType::Lowerroman,
            (
                EnumType::UpperAmbiguousI | EnumType::UpperAmbiguousC,
                Some(EnumType::Upperalpha | EnumType::Loweralpha),
            ) => EnumType::Upperalpha,
            (
                EnumType::LowerAmbiguousI | EnumType::LowerAmbiguousC,
                Some(EnumType::Upperalpha | EnumType::Loweralpha),
            ) => EnumType::Loweralpha,
            (EnumType::UpperAmbiguousI, None) => EnumType::Upperroman,
            (EnumType::LowerAmbiguousI, None) => EnumType::Lowerroman,
            (EnumType::UpperAmbiguousC, None) => EnumType::Upperalpha,
            (EnumType::LowerAmbiguousC, None) => EnumType::Loweralpha,
            (marker_type, _) => marker_type,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        alphabetic_value, enumerator_type, enumerator_value, resolve_enumerator_type, roman_value,
        EnumType,
    };
    use crate::{parse, parser_errors::ParserError};
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
    fn enumerator_type_identifies_supported_marker_types() {
        assert_eq!(enumerator_type("12"), Ok(EnumType::Arabic));
        assert_eq!(enumerator_type("ABC"), Ok(EnumType::Upperalpha));
        assert_eq!(enumerator_type("abc"), Ok(EnumType::Loweralpha));
        assert_eq!(enumerator_type("XL"), Ok(EnumType::Upperroman));
        assert_eq!(enumerator_type("xl"), Ok(EnumType::Lowerroman));
        assert_eq!(enumerator_type("I"), Ok(EnumType::UpperAmbiguousI));
        assert_eq!(enumerator_type("i"), Ok(EnumType::LowerAmbiguousI));
        assert_eq!(enumerator_type("C"), Ok(EnumType::UpperAmbiguousC));
        assert_eq!(enumerator_type("c"), Ok(EnumType::LowerAmbiguousC));
    }

    #[test]
    fn enumerator_type_rejects_invalid_markers() {
        for marker in ["a1", "A!", "aB", "Xl"] {
            assert_eq!(
                enumerator_type(marker),
                Err(ParserError::ListMarkerError {
                    marker: marker.to_string(),
                })
            );
        }
    }

    #[test]
    fn alphabetic_and_roman_values_convert_supported_values() {
        assert_eq!(alphabetic_value("C"), Some(3));
        assert_eq!(alphabetic_value("z"), Some(26));
        assert_eq!(roman_value("XL"), Some(40));
        assert_eq!(roman_value("xl"), Some(40));
    }

    #[test]
    fn enumerator_value_converts_supported_marker_values() {
        assert_eq!(enumerator_value("12", EnumType::Arabic), Ok(12));
        assert_eq!(enumerator_value("C", EnumType::Upperalpha), Ok(3));
        assert_eq!(enumerator_value("z", EnumType::Loweralpha), Ok(26));
        assert_eq!(enumerator_value("XL", EnumType::Upperroman), Ok(40));
        assert_eq!(enumerator_value("xl", EnumType::Lowerroman), Ok(40));
    }

    #[test]
    fn resolve_enumerator_type_uses_initial_marker_rules() {
        assert_eq!(
            resolve_enumerator_type(EnumType::UpperAmbiguousI, None, None),
            EnumType::Upperroman
        );
        assert_eq!(
            resolve_enumerator_type(EnumType::LowerAmbiguousC, None, None),
            EnumType::Loweralpha
        );
    }

    #[test]
    fn resolve_enumerator_type_uses_existing_list_type() {
        assert_eq!(
            resolve_enumerator_type(
                EnumType::UpperAmbiguousI,
                Some(EnumType::Upperalpha),
                Some("H"),
            ),
            EnumType::Upperalpha
        );
        assert_eq!(
            resolve_enumerator_type(EnumType::UpperAmbiguousC, Some(EnumType::Lowerroman), None),
            EnumType::Upperroman
        );
    }

    #[test]
    fn resolve_ambiguous_i_uses_the_previous_marker() {
        assert_eq!(
            resolve_enumerator_type(
                EnumType::UpperAmbiguousI,
                Some(EnumType::Upperalpha),
                Some("G"),
            ),
            EnumType::Upperroman
        );
        assert_eq!(
            resolve_enumerator_type(
                EnumType::LowerAmbiguousI,
                Some(EnumType::Loweralpha),
                Some("g"),
            ),
            EnumType::Lowerroman
        );
        assert_eq!(
            resolve_enumerator_type(
                EnumType::LowerAmbiguousI,
                Some(EnumType::Loweralpha),
                Some("h"),
            ),
            EnumType::Loweralpha
        );
    }

    #[test]
    fn ambiguous_i_starts_a_roman_list() {
        assert_eq!(list_types("I. first\nII. second\n"), ["upperroman"]);
    }

    #[test]
    fn ambiguous_c_starts_an_alpha_list() {
        assert_eq!(list_types("C. first\nD. second\n"), ["upperalpha"]);
    }

    #[test]
    fn ambiguous_markers_use_the_existing_list_style() {
        assert_eq!(list_types("H. first\nI. second\n"), ["upperalpha"]);
        assert_eq!(list_types("I. first\nC. second\n"), ["upperroman"]);
    }

    #[test]
    fn ambiguous_i_starts_a_roman_list_unless_preceded_by_h() {
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

    #[test]
    fn enumerator_value_rejects_invalid_values() {
        for (value, enumtype) in [
            ("not-a-number", EnumType::Arabic),
            ("A1", EnumType::Upperalpha),
            ("invalid", EnumType::Lowerroman),
        ] {
            assert_eq!(
                enumerator_value(value, enumtype),
                Err(ParserError::ListMarkerError {
                    marker: value.to_string(),
                })
            );
        }
    }
}
