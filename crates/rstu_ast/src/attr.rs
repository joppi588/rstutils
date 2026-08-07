// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeType {
    Integer(i32),
    Float(f64),
    String(String),
    Usize(usize),
}

impl AttributeType {
    pub(crate) fn as_str(&self) -> Option<&str> {
        match self {
            AttributeType::String(value) => Some(value.as_str()),
            _ => None,
        }
    }
}

impl From<String> for AttributeType {
    fn from(value: String) -> Self {
        AttributeType::String(value)
    }
}

impl From<&str> for AttributeType {
    fn from(value: &str) -> Self {
        AttributeType::String(value.to_owned())
    }
}

impl From<usize> for AttributeType {
    fn from(value: usize) -> Self {
        AttributeType::Usize(value)
    }
}

impl From<i32> for AttributeType {
    fn from(value: i32) -> Self {
        AttributeType::Integer(value)
    }
}

impl From<f64> for AttributeType {
    fn from(value: f64) -> Self {
        AttributeType::Float(value)
    }
}
