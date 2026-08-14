// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParserError {
    StartAtOutOfBounds {
        start_at: usize,
        token_count: usize,
    },
    SectionTitleMissingClosingAfterOpening {
        opening_index: usize,
    },
    SectionTitleUnbalancedStyle {
        opening_index: usize,
        opening_style: String,
        closing_style: String,
    },
    InvalidPlainText {
        start_at: usize,
    },
    UnexpectedToken {
        expected: String,
        found: String,
    },
    InlineMissingClosing {
        markup: String,
        start_at: usize,
    },
    ListStyleError {
        marker: String,
        conflicting_marker: String,
    },
}

pub(crate) static EXPECT_NEWLINE: &str = "There is at least one newline at the end of tokens.";
