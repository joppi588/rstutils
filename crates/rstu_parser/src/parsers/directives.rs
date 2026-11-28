// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::block::parse_block;
use crate::is_rst_directive;
use crate::parser_errors::ParserError;
use crate::parsers::lists::parse_field_list;
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
        let base_indent = stream.consume().len();
        directive.with_attr("indent", base_indent);
        if stream.token_at_cursor().kind == TK::Field {
            directive.push_child(parse_field_list(stream)?);
        }
        let directive_type = directive.borrow().attributes.get_str("directive_type");
        if !matches!(stream.token_at_cursor().kind, TK::Dedent | TK::EoF) {
            if is_rst_directive(directive_type.as_deref().unwrap_or_default()) {
                let content = parse_block(stream)?;
                directive.push_child(content);
            } else {
                let raw_content = stream.consume_indented_text(base_indent);
                directive.with_attr("raw_content", raw_content);
            }
        }
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
    use crate::{parse_with_config, ParserConfig};

    #[test]
    fn config_selects_rst_parsing_and_is_restored() {
        let source = ".. custom::\n\n   text\n";
        let config = ParserConfig {
            rst_directives: vec!["custom".to_owned()],
        };

        let parsed = parse_with_config(source, &config).unwrap();
        let directive = parsed.borrow().children[0].clone();
        assert!(directive
            .borrow()
            .attributes
            .get_str("raw_content")
            .is_none());

        let parsed = crate::parse(source).unwrap();
        let directive = parsed.borrow().children[0].clone();
        assert_eq!(
            directive
                .borrow()
                .attributes
                .get_str("raw_content")
                .as_deref(),
            Some("text\n")
        );
    }

    #[test]
    fn parse_substitution_directive_marker() {
        let mut stream = tokenize(".. |name| replace:: target\n");
        stream.consume(); //synthetic blank line

        let directive = parse_directive(&mut stream).unwrap();
        let directive = directive.borrow();

        assert_eq!(
            directive.attributes.get_str("substitution").as_deref(),
            Some("name")
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
