// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use serde_json::Value;

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

    pub(crate) fn to_json_value(&self) -> Value {
        match self {
            AttributeType::Integer(value) => Value::from(*value),
            AttributeType::Float(value) => Value::from(*value),
            AttributeType::String(value) => Value::String(value.clone()),
            AttributeType::Usize(value) => Value::from(*value as u64),
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

macro_rules! impl_from_signed {
    ($($type:ty),* $(,)?) => {
        $(
            impl From<$type> for AttributeType {
                fn from(value: $type) -> Self {
                    AttributeType::Integer(value as i32)
                }
            }
        )*
    };
}

macro_rules! impl_from_unsigned {
    ($($type:ty),* $(,)?) => {
        $(
            impl From<$type> for AttributeType {
                fn from(value: $type) -> Self {
                    AttributeType::Usize(value as usize)
                }
            }
        )*
    };
}

impl_from_signed!(i8, i16, i32);
impl_from_unsigned!(u8, u16, u32, usize);

impl From<i64> for AttributeType {
    fn from(value: i64) -> Self {
        AttributeType::Integer(value as i32)
    }
}

impl From<isize> for AttributeType {
    fn from(value: isize) -> Self {
        AttributeType::Integer(value as i32)
    }
}

impl From<u64> for AttributeType {
    fn from(value: u64) -> Self {
        AttributeType::Usize(value as usize)
    }
}

impl From<f32> for AttributeType {
    fn from(value: f32) -> Self {
        AttributeType::Float(value as f64)
    }
}

impl From<f64> for AttributeType {
    fn from(value: f64) -> Self {
        AttributeType::Float(value)
    }
}

impl From<bool> for AttributeType {
    fn from(value: bool) -> Self {
        AttributeType::String(value.to_string())
    }
}
