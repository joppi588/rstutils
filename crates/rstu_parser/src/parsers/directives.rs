// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::block::parse_block;
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_directive(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.token_at_cursor().kind, TK::Directive);

    let directive = AstNode::new_ref(NodeClass::Directive);
    parse_directive_header(stream, &directive);

    if stream.token_at_cursor().kind == TK::BlankLine {
        directive.push_blank_lines(stream.consume().len())
    }

    if stream.token_at_cursor().kind == TK::Indent {
        let content = parse_block(stream)?;
        directive.push_child(content);
    }
    Ok(directive)
}

fn parse_directive_header(stream: &mut TokenStream, directive: &NodeRef) {
    let directive_marker = stream.consume().lexeme;
    let marker_content = directive_marker
        .strip_prefix(".. ")
        .and_then(|marker| marker.strip_suffix("::"))
        .unwrap_or_default();
    let marker_parts: Vec<_> = marker_content
        .split('|')
        .filter(|part| !part.is_empty())
        .collect();

    if marker_parts.len() >= 2 {
        directive.with_attr("substitution", marker_parts[0]);
    }
    directive.with_attr(
        "directive_type",
        marker_parts.last().copied().unwrap_or_default().trim(),
    );

    let mut directive_arguments = String::new();
    while !matches!(stream.token_at_cursor().kind, TK::NewLine | TK::EoF) {
        directive_arguments.push_str(&stream.consume().lexeme);
    }
    let directive_arguments = directive_arguments.trim();
    if !directive_arguments.is_empty() {
        directive.with_attr("directive_arguments", directive_arguments);
    }
    stream.consume_newline();
}

#[cfg(test)]
mod tests {
    use super::parse_directive;
    use crate::lexer::tokenize;

    #[test]
    fn parse_substitution_directive_marker() {
        let mut stream = tokenize(".. | name | replace:: target\n");

        let directive = parse_directive(&mut stream).unwrap();
        let directive = directive.borrow();

        assert_eq!(
            directive.attributes.get_str("substitution").as_deref(),
            Some(" name ")
        );
        assert_eq!(
            directive.attributes.get_str("directive_type").as_deref(),
            Some("replace")
        );
        assert_eq!(
            directive
                .attributes
                .get_str("directive_arguments")
                .as_deref(),
            Some("target")
        );
    }
}
