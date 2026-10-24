// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token_stream::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
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
    LiteralBlockError {
        message: String,
        position: Position,
    },
    MissingBlankLineError {
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
                    "{position}: Expected dedent of {expected}, found {found}"
                )
            }
            Self::InlineMissingClosing {
                markup, position, ..
            } => write!(
                formatter,
                "{position}: inline {markup} markup is not closed."
            ),
            Self::ListEndError { position } => {
                write!(
                    formatter,
                    "{position}: list item is not followed by an indented body"
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
                "{position}: list marker {conflicting_marker:?} conflicts with {marker:?}"
            ),
            Self::LiteralBlockError { message, position } => {
                write!(formatter, "{position}: {message}")
            }
            Self::MissingBlankLineError { entity,position } => {
                write!(
                    formatter,
                    "{position}: {entity} is not followed by a blank line."
                )
            }
            Self::OptionError { text, position } => {
                write!(formatter, "{position}: invalid option {text:?}")
            }
            Self::SectionTitleMissingClosingAfterOpening { position, .. } => {
                write!(
                    formatter,
                    "{position}: section title has no closing underline."
                )
            }
            Self::SectionTitleUnbalancedStyle {
                opening_style,
                closing_style,
                position,
                ..
            } => write!(
                formatter,
                "{position}: section underline {closing_style:?} does not match overline {opening_style:?}"
            ),
            Self::UnexpectedIndentError { position } => {
                write!(formatter, "{position}: unexpected indentation")
            }
            Self::UnexpectedToken {
                expected,
                found,
                position,
                ..
            } => write!(formatter, "{position}: expected {expected}, found {found}"),
        }
    }
}
