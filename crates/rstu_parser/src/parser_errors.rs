// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use crate::token_stream::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    StartAtOutOfBounds {
        start_at: usize,
        token_count: usize,
    },
    SectionTitleMissingClosingAfterOpening {
        opening_index: usize,
        position: Position,
    },
    SectionTitleUnbalancedStyle {
        opening_index: usize,
        opening_style: String,
        closing_style: String,
        position: Position,
    },
    InvalidPlainText {
        start_at: usize,
    },
    UnexpectedToken {
        expected: String,
        found: String,
        index: usize,
        position: Position,
    },
    InlineMissingClosing {
        markup: String,
        start_at: usize,
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
    ListMissingBlankLineError {},
    UnexpectedIndentError {
        position: Position,
    },
    InvalidDedent {
        expected: usize,
        is: usize,
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
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartAtOutOfBounds {
                start_at,
                token_count,
            } => write!(
                formatter,
                "start index {start_at} is out of bounds for {token_count} tokens"
            ),
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
            Self::InvalidPlainText { start_at } => {
                write!(formatter, "invalid plain text at token {start_at}")
            }
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
                "{position}: inline {markup} markup is not closed"
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
            Self::ListMissingBlankLineError {} => {
                write!(formatter, "list is not followed by a blank line")
            }
            Self::UnexpectedIndentError { position } => {
                write!(formatter, "{position}: unexpected indentation")
            }
            Self::InvalidDedent { expected, is } => {
                write!(formatter, "invalid dedent: expected {expected}, found {is}")
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
        }
    }
}

impl std::error::Error for ParserError {}

pub(crate) static EXPECT_NEWLINE: &str = "There is at least one newline at the end of tokens.";
