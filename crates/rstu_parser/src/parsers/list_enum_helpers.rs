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

#[cfg(test)]
mod tests {
    use super::{enumerator_type, enumerator_value, EnumMarkerType};
    use crate::parser_errors::ParserError;

    #[test]
    fn enumerator_type_identifies_supported_marker_types() {
        // GIVEN markers using each supported enumeration style
        // WHEN their enumeration types are detected
        // THEN each marker is assigned its matching type
        assert_eq!(enumerator_type("12"), Ok(EnumMarkerType::Arabic));
        assert_eq!(enumerator_type("ABC"), Ok(EnumMarkerType::Upperalpha));
        assert_eq!(enumerator_type("abc"), Ok(EnumMarkerType::Loweralpha));
        assert_eq!(enumerator_type("XL"), Ok(EnumMarkerType::Upperroman));
        assert_eq!(enumerator_type("xl"), Ok(EnumMarkerType::Lowerroman));
    }

    #[test]
    fn enumerator_type_rejects_invalid_markers() {
        // GIVEN markers that contain invalid or mixed characters
        // WHEN their enumeration types are detected
        // THEN a list marker error containing the original marker is returned
        for marker in ["a1", "A!", "aB"] {
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
        assert_eq!(enumerator_value("12", EnumMarkerType::Arabic), Ok(12));
        assert_eq!(enumerator_value("C", EnumMarkerType::Upperalpha), Ok(3));
        assert_eq!(enumerator_value("z", EnumMarkerType::Loweralpha), Ok(26));
        assert_eq!(enumerator_value("XL", EnumMarkerType::Upperroman), Ok(40));
        assert_eq!(enumerator_value("xl", EnumMarkerType::Lowerroman), Ok(40));
    }

    #[test]
    fn enumerator_value_rejects_invalid_values() {
        // GIVEN values that cannot be converted for their requested type
        // WHEN their numeric values are converted
        // THEN a list marker error containing the original value is returned
        for (value, enumtype) in [
            ("not-a-number", EnumMarkerType::Arabic),
            ("A1", EnumMarkerType::Upperalpha),
            ("invalid", EnumMarkerType::Lowerroman),
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
