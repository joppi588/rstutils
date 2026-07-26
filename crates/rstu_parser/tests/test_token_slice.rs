// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_parser::token::TokenKind;
use rstu_parser::token_slice::{ScanDirection, TokenSlice};

#[test]
fn from_string_and_to_text_roundtrip() {
    let view = TokenSlice::from_string("Hello World\n");

    assert_eq!(view.to_text(), "Hello World\n");
}

#[test]
fn supports_slicing_and_element_access() {
    let view = TokenSlice::from_string("Hello World\n");
    let hello = view.slice(0..1).expect("slice should be valid");
    let world = view.slice(2..3).expect("slice should be valid");

    assert_eq!(
        hello.get(0).map(|token| token.lexeme.as_str()),
        Some("Hello")
    );
    assert_eq!(
        world.get(0).map(|token| token.lexeme.as_str()),
        Some("World")
    );
}

#[test]
fn supports_cursor_navigation() {
    let mut view = TokenSlice::from_string("A B\n");

    assert_eq!(view.current().map(|token| token.lexeme.as_str()), Some("A"));
    assert!(view.advance());
    assert_eq!(view.current().map(|token| token.lexeme.as_str()), Some(" "));
    assert!(view.set_cursor(2));
    assert_eq!(view.current().map(|token| token.lexeme.as_str()), Some("B"));
    assert!(view.retreat());
    assert_eq!(view.current().map(|token| token.lexeme.as_str()), Some(" "));
}

#[test]
fn forward_scan_until_next_kind() {
    let mut view = TokenSlice::from_string("Hello World\nTail");
    assert!(view.set_cursor(2)); // World

    let scan = view.until_next_kind(TokenKind::NewLine, ScanDirection::Forward);

    assert_eq!(scan.to_text(), "World");
}

#[test]
fn backward_scan_until_next_kind() {
    let mut view = TokenSlice::from_string("Left\nRight");
    assert!(view.set_cursor(3)); // Right

    let scan = view.until_next_kind(TokenKind::NewLine, ScanDirection::Backward);

    assert_eq!(scan.to_text(), "Right");
}
