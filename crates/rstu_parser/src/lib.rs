// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

pub mod lexer;
pub mod token;

use rstu_ast::{ElementKind, Node};

use crate::lexer::tokenize;

pub fn parse(input: &str) -> &Node {
    let tokens = tokenize(input);
    let doc: &Node = Node::new(ElementKind::root);

    doc
}
