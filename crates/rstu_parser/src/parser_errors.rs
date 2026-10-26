// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token_stream::Position;
use strum::EnumDiscriminants;

#[derive(Debug, Clone, PartialEq, Eq, EnumDiscriminants)]
#[strum_discriminants(name(ParserErrorKind))]
pub enum ParserError {
    DedentToSmall {
        position: Position,
        expected: usize,
        found: usize,
    },
    InlineMissingClosing {
        markup: String,
        position: Position,
    },
    ListEndError {
        position: Position,
    },
    ListMarkerError {
        marker: String,
        position: Position,
    },
    ListStyleError {
        marker: String,
        conflicting_marker: String,
        position: Position,
    },
    LiteralBlockInconsistentQuoteMarker {
        expected: char,
        found: char,
        position: Position,
    },
    LiteralBlockInvalidQuoteMarker {
        marker: char,
        position: Position,
    },
    LiteralBlockMissingBlankLine {
        position: Position,
    },
    LiteralBlockMissingContent {
        position: Position,
    },
    MissingFinalBlankLineError {
        entity: String,
        position: Position,
    },
    OptionError {
        text: String,
        position: Position,
    },
    SectionTitleMissingClosingAfterOpening {
        position: Position,
    },
    SectionTitleUnbalancedStyle {
        opening_style: String,
        closing_style: String,
        position: Position,
    },
    UnexpectedIndentError {
        position: Position,
    },
    UnexpectedToken {
        expected: String,
        found: String,
        position: Position,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DedentToSmall {
                position,
                expected,
                found,
            } => {
                write!(
                    formatter,
                    "{position}: Expected dedent of {expected}, found {found}."
                )
            }
            Self::InlineMissingClosing {
                markup, position, ..
            } => write!(
                formatter,
                "{position}: Inline markup ({markup}) is not closed."
            ),
            Self::ListEndError { position } => {
                write!(
                    formatter,
                    "{position}: List item is not followed by an indented body or blank line."
                )
            }
            Self::ListMarkerError { marker, position } => {
                write!(formatter, "{position}: invalid list marker {marker:?}")
            }
            Self::ListStyleError {
                marker,
                conflicting_marker,
                position,
            } => write!(
                formatter,
                "{position}: List marker {conflicting_marker:?} conflicts with {marker:?}."
            ),
            Self::LiteralBlockInconsistentQuoteMarker {
                expected,
                found,
                position,
            } => write!(
                formatter,
                "{position}: Quoted literal block line starts with {found:?}, expected {expected:?}."
            ),
            Self::LiteralBlockInvalidQuoteMarker { marker, position } => write!(
                formatter,
                "{position}: {marker:?} is not a valid quoted literal block marker."
            ),
            Self::LiteralBlockMissingBlankLine { position } => write!(
                formatter,
                "{position}: Literal block marker is not followed by a blank line."
            ),
            Self::LiteralBlockMissingContent { position } => write!(
                formatter,
                "{position}: Literal block expected after literal marker."
            ),
            Self::MissingFinalBlankLineError { entity,position } => {
                write!(
                    formatter,
                    "{position}: {entity} is not followed by a blank line."
                )
            }
            Self::OptionError { text, position } => {
                write!(formatter, "{position}: Invalid option {text:?}.")
            }
            Self::SectionTitleMissingClosingAfterOpening { position, .. } => {
                write!(
                    formatter,
                    "{position}: Section title has no closing underline."
                )
            }
            Self::SectionTitleUnbalancedStyle {
                opening_style,
                closing_style,
                position,
                ..
            } => write!(
                formatter,
                "{position}: Section underline {closing_style:?} does not match overline {opening_style:?}"
            ),
            Self::UnexpectedIndentError { position } => {
                write!(formatter, "{position}: Unexpected indentation")
            }
            Self::UnexpectedToken {
                expected,
                found,
                position,
                ..
            } => write!(formatter, "{position}: Expected {expected}, found {found}"),
        }
    }
}
