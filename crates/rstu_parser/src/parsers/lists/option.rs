// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use super::{consume_trailing_blank_lines, parse_item_body};
use crate::parser_errors::ParserError;
use crate::token::{
    TokenCategory as TC, TokenKind as TK, DOS_OPTIONS_MATCH, LONG_OPTIONS_MATCH,
    SHORT_OPTIONS_MATCH,
};
use crate::token_stream::TokenStream;
use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};
use std::debug_assert_matches;

pub(crate) fn parse_option_list(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::Option);

    let list = AstNode::new_ref(NodeClass::OptionList);

    while stream.token_at_cursor().kind.is(TC::OPTION_MARKER) {
        let option_item = AstNode::new_ref(NodeClass::OptionListItem);
        let mut dedent_len: usize = 0;
        let mut text = String::new();

        loop {
            let token = stream.token_at_cursor();
            if token.kind == TK::NewLine {
                break;
            }

            dedent_len += token.len();
            if token.kind == TK::Spaces && token.len() >= 2 {
                stream.consume();
                break;
            }

            text.push_str(&token.lexeme);
            stream.consume();
        }
        let option_group = parse_option_group(&text)?;
        option_item.push_child(option_group);
        option_item.push_child(parse_item_body(stream, dedent_len)?);
        list.push_child(option_item);
        consume_trailing_blank_lines(stream, &list);
    }

    Ok(list)
}

fn parse_option_group(text: &str) -> Result<NodeRef, ParserError> {
    let option_group = AstNode::new_ref(NodeClass::OptionGroup);
    for text in text.split(',') {
        option_group.push_child(parse_option(text)?);
    }
    Ok(option_group)
}

fn parse_option(text: &str) -> Result<NodeRef, ParserError> {
    let option = AstNode::new_ref(NodeClass::Option);
    let captures = [
        &SHORT_OPTIONS_MATCH,
        &LONG_OPTIONS_MATCH,
        &DOS_OPTIONS_MATCH,
    ]
    .iter()
    .find_map(|regex| regex.captures(text));

    match captures {
        Some(caps) => {
            for (name, group) in [("arg", 3), ("delimiter", 2)] {
                if !caps[group].is_empty() {
                    option.with_attr(name, caps[group].to_string());
                }
            }
            option.with_attr("flag", caps[1].to_string());
            Ok(option)
        }
        None => Err(ParserError::OptionError {
            text: text.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::parse_option;

    #[test]
    fn parse_short_option_argument() {
        let result = parse_option("-a arg").expect("Can be parsed.");
        let attrs = &result.borrow().attributes;

        assert_eq!(attrs.get_str("flag"), Some("-a".to_string()));
        assert_eq!(attrs.get_str("delimiter"), Some(" ".to_string()));
        assert_eq!(attrs.get_str("arg"), Some("arg".to_string()));
    }

    #[test]
    fn parse_old_gnu_option_argument() {
        let result = parse_option("+b file").expect("Can be parsed.");
        let attrs = &result.borrow().attributes;

        assert_eq!(attrs.get_str("flag"), Some("+b".to_string()));
        assert_eq!(attrs.get_str("delimiter"), Some(" ".to_string()));
        assert_eq!(attrs.get_str("arg"), Some("file".to_string()));
    }
}
