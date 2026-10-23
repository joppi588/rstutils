// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token_stream::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    SectionTitleMissingClosingAfterOpening {
        position: Position,
    },
    SectionTitleUnbalancedStyle {
        opening_style: String,
        closing_style: String,
        position: Position,
    },
    UnexpectedToken {
        expected: String,
        found: String,
        position: Position,
    },
    InlineMissingClosing {
        markup: String,
        position: Position,
    },
    ListMarkerError {
        marker: String,
    },
    ListStyleError {
        marker: String,
        conflicting_marker: String,
        position: Position,
    },
    ListEndError {
        position: Position,
    },
    MissingBlankLineError {},
    UnexpectedIndentError {
        position: Position,
    },
    LiteralBlockError {
        message: String,
        position: Position,
    },
    OptionError {
        text: String,
    },
    BlockQuoteMissingBlankLineError {
        position: Position,
    },
    DedentToSmall {
        position: Position,
        expected: usize,
        found: usize,
    },
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SectionTitleMissingClosingAfterOpening { position, .. } => {
                write!(
                    formatter,
                    "{position}: section title has no closing underline"
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
            Self::UnexpectedToken {
                expected,
                found,
                position,
                ..
            } => write!(formatter, "{position}: expected {expected}, found {found}"),
            Self::InlineMissingClosing {
                markup, position, ..
            } => write!(
                formatter,
                "{position}: inline {markup} markup is not closed."
            ),
            Self::ListMarkerError { marker } => write!(formatter, "invalid list marker {marker:?}"),
            Self::ListStyleError {
                marker,
                conflicting_marker,
                position,
            } => write!(
                formatter,
                "{position}: list marker {conflicting_marker:?} conflicts with {marker:?}"
            ),
            Self::ListEndError { position } => {
                write!(
                    formatter,
                    "{position}: list item is not followed by an indented body"
                )
            }
            Self::MissingBlankLineError {} => {
                write!(formatter, "list is not followed by a blank line")
            }
            Self::UnexpectedIndentError { position } => {
                write!(formatter, "{position}: unexpected indentation")
            }
            Self::LiteralBlockError { message, position } => {
                write!(formatter, "{position}: {message}")
            }
            Self::OptionError { text } => write!(formatter, "invalid option {text:?}"),
            Self::BlockQuoteMissingBlankLineError { position } => {
                write!(
                    formatter,
                    "{position}: block quote is not followed by a blank line"
                )
            }
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
        }
    }
}
