// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::ParserError;
use crate::token::{LONG_OPTIONS_MATCH_GROUPS, SHORT_OPTIONS_MATCH_GROUPS};
use regex::Regex;
use rstu_ast::NodeRef;
use rstu_ast::{AstNode, NodeClass, NodeRefExt};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum EnumType {
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

pub(super) fn alphabetic_value(value: &str) -> Option<usize> {
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

pub(super) fn roman_value(value: &str) -> Option<usize> {
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

pub(super) fn enumerator_parts(marker: &str) -> (&str, &str, &str) {
    let (prefix, value, suffix) = if marker.starts_with('(') && marker.ends_with(')') {
        ("(", &marker[1..marker.len() - 1], ")")
    } else {
        let split_at = marker.len().saturating_sub(1);
        ("", &marker[..split_at], &marker[split_at..])
    };
    (prefix, value, suffix)
}

pub(super) fn enumerator_value(value: &str, enumtype: EnumType) -> Result<usize, ParserError> {
    let converted_value = match enumtype {
        EnumType::Arabic => value.parse().ok(),
        EnumType::Upperalpha | EnumType::Loweralpha => alphabetic_value(value),
        EnumType::Upperroman | EnumType::Lowerroman => roman_value(value),
        EnumType::UpperAmbiguousI
        | EnumType::LowerAmbiguousI
        | EnumType::UpperAmbiguousC
        | EnumType::LowerAmbiguousC => None,
    };
    return converted_value.ok_or_else(|| ParserError::ListMarkerError {
        marker: (value.to_string()),
    });
}

pub(super) fn enumerator_type(value: &str) -> Result<EnumType, ParserError> {
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

pub(super) fn resolve_enumerator_type(
    marker_type: EnumType,
    list_type: Option<EnumType>,
    previous_value: Option<&str>,
) -> EnumType {
    match marker_type {
        EnumType::UpperAmbiguousI if previous_value != Some("H") => {
            return EnumType::Upperroman;
        }
        EnumType::LowerAmbiguousI if previous_value != Some("h") => {
            return EnumType::Lowerroman;
        }
        _ => {}
    }

    match (marker_type, list_type) {
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
    }
}

pub(super) fn parse_option_group(text: &str) -> Result<NodeRef, ParserError> {
    let option_group = AstNode::new_ref(NodeClass::OptionGroup);
    for option in text.split(',') {
        let option = parse_option(option)?;
        option_group.push_child(option);
    }
    Ok(option_group)
}

fn parse_option(text: &str) -> Result<NodeRef, ParserError> {
    let option = AstNode::new_ref(NodeClass::Option);

    let captures = [SHORT_OPTIONS_MATCH_GROUPS, LONG_OPTIONS_MATCH_GROUPS]
        .iter()
        .find_map(|pattern| {
            Regex::new(pattern)
                .expect("valid option regex")
                .captures(text)
        });

    match captures {
        Some(caps) => {
            option.with_attr("arg", caps[3].to_string());
            option.with_attr("delimiter", caps[2].to_string());
            option.with_attr("flag", caps[1].to_string());
            Ok(option)
        }
        None => Err(ParserError::NoOptionFound {
            text: text.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{enumerator_type, enumerator_value, resolve_enumerator_type, EnumType};
    use crate::{parser_errors::ParserError, parsers::list_helpers::parse_option_group};

    #[test]
    fn enumerator_type_identifies_supported_marker_types() {
        // GIVEN markers using each supported enumeration style
        // WHEN their enumeration types are detected
        // THEN each marker is assigned its matching type
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
        // GIVEN markers that contain invalid or mixed characters
        // WHEN their enumeration types are detected
        // THEN a list marker error containing the original marker is returned
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
    fn enumerator_value_converts_supported_marker_values() {
        // GIVEN valid values for each supported enumeration type
        // WHEN their numeric values are converted
        // THEN the corresponding ordinal value is returned
        assert_eq!(enumerator_value("12", EnumType::Arabic), Ok(12));
        assert_eq!(enumerator_value("C", EnumType::Upperalpha), Ok(3));
        assert_eq!(enumerator_value("z", EnumType::Loweralpha), Ok(26));
        assert_eq!(enumerator_value("XL", EnumType::Upperroman), Ok(40));
        assert_eq!(enumerator_value("xl", EnumType::Lowerroman), Ok(40));
    }

    #[test]
    fn resolve_enumerator_type_uses_initial_marker_rules() {
        // GIVEN ambiguous markers starting a list
        // WHEN their types are resolved without an existing list type
        // THEN I is Roman and C is alphabetic
        assert_eq!(
            resolve_enumerator_type(EnumType::UpperAmbiguousI, None, None,),
            EnumType::Upperroman
        );
        assert_eq!(
            resolve_enumerator_type(EnumType::LowerAmbiguousC, None, None,),
            EnumType::Loweralpha
        );
    }

    #[test]
    fn resolve_enumerator_type_uses_existing_list_type() {
        // GIVEN ambiguous markers inside established lists
        // WHEN their types are resolved against the list type
        // THEN they use the existing list family and marker case
        assert_eq!(
            resolve_enumerator_type(
                EnumType::UpperAmbiguousI,
                Some(EnumType::Upperalpha),
                Some("H"),
            ),
            EnumType::Upperalpha
        );
        assert_eq!(
            resolve_enumerator_type(EnumType::UpperAmbiguousC, Some(EnumType::Lowerroman), None,),
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
    fn enumerator_value_rejects_invalid_values() {
        // GIVEN values that cannot be converted for their requested type
        // WHEN their numeric values are converted
        // THEN a list marker error containing the original value is returned
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

    #[test]
    fn parse_short_option_argument() {
        // GIVEN A short option with an argument
        // WHEN the option is parsed
        // An option group is returned

        let option = "-a arg";
        let result = parse_option_group(option).expect("Can be parsed.");
        let child = &result.borrow().children[0];
        let attrs = &child.borrow().attributes;

        assert_eq!(attrs.get_str("flag"), Some("-a".to_string()));
        assert_eq!(attrs.get_str("delimiter"), Some(" ".to_string()));
        assert_eq!(attrs.get_str("arg"), Some("arg".to_string()));
    }

    #[test]
    fn parse_old_gnu_option_argument() {
        let result = parse_option_group("+b file").expect("Can be parsed.");
        let child = &result.borrow().children[0];
        let attrs = &child.borrow().attributes;

        assert_eq!(attrs.get_str("flag"), Some("+b".to_string()));
        assert_eq!(attrs.get_str("delimiter"), Some(" ".to_string()));
        assert_eq!(attrs.get_str("arg"), Some("file".to_string()));
    }
}
