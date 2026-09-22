// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::parser_errors::ParserError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum EnumMarkerType {
    Arabic,
    Upperalpha,
    Loweralpha,
    Upperroman,
    Lowerroman,
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

pub(super) fn enumerator_value(
    value: &str,
    enumtype: EnumMarkerType,
) -> Result<usize, ParserError> {
    let converted_value = match enumtype {
        EnumMarkerType::Arabic => value.parse().ok(),
        EnumMarkerType::Upperalpha | EnumMarkerType::Loweralpha => alphabetic_value(value),
        EnumMarkerType::Upperroman | EnumMarkerType::Lowerroman => roman_value(value),
    };
    return converted_value.ok_or_else(|| ParserError::ListMarkerError {
        marker: (value.to_string()),
    });
}

pub(super) fn enumerator_type(value: &str) -> Result<EnumMarkerType, ParserError> {
    if value.chars().all(|character| character.is_ascii_digit()) {
        Ok(EnumMarkerType::Arabic)
    } else if roman_value(value).is_some()
        && value.chars().all(|character| {
            matches!(
                character.to_ascii_uppercase(),
                //                'I' | 'V' | 'X' | 'L' | 'C' | 'D' | 'M'
                'I' | 'V' | 'X' | 'L' // TODO: Allow Values > 100 (C)
            )
        })
    {
        if value
            .chars()
            .all(|character| character.is_ascii_uppercase())
        {
            Ok(EnumMarkerType::Upperroman)
        } else {
            Ok(EnumMarkerType::Lowerroman)
        }
    } else if value
        .chars()
        .all(|character| character.is_ascii_uppercase())
    {
        Ok(EnumMarkerType::Upperalpha)
    } else if value
        .chars()
        .all(|character| character.is_ascii_lowercase())
    {
        Ok(EnumMarkerType::Loweralpha)
    } else {
        Err(ParserError::ListMarkerError {
            marker: (value.to_string()),
        })
    }
}
