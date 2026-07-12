// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Document {
    pub blocks: Vec<Block>,
}

impl Document {
    pub fn new(blocks: Vec<Block>) -> Self {
        Self { blocks }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Block {
    Heading(Heading),
    Comment(Comment),
    Directive(Directive),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Heading {
    pub title: Sentence,
    pub underline: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    pub text: Sentence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Directive {
    pub name: String,
    pub argument: Option<Sentence>,
    pub body: IndentedBlock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndentedBlock {
    pub indent: String,
    pub lines: Vec<Sentence>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sentence {
    pub text: String,
}

impl Sentence {
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}
