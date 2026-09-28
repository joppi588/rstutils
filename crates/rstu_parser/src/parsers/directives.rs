// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use super::{block::parse_block, list::parse_field_list};
use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;
use std::debug_assert_matches;

pub(crate) fn parse_directive(stream: &mut TokenStream) -> Result<NodeRef, ParserError> {
    debug_assert_matches!(stream.kind_at_cursor(), TK::Directive);

    let directive_marker = stream.consume().lexeme;
    let marker_content = directive_marker
        .strip_prefix(".. ")
        .and_then(|marker| marker.strip_suffix("::"))
        .unwrap_or_default();
    let marker_parts: Vec<_> = marker_content
        .split('|')
        .filter(|part| !part.is_empty())
        .collect();

    let directive = AstNode::new_ref(NodeClass::Directive);
    if marker_parts.len() >= 2 {
        directive.with_attr("substitution", marker_parts[0]);
    }
    directive.with_attr(
        "directive_type",
        marker_parts.last().copied().unwrap_or_default().trim(),
    );

    let mut directive_arguments = String::new();
    while !matches!(stream.kind_at_cursor(), TK::NewLine | TK::EoF) {
        directive_arguments.push_str(&stream.consume().lexeme);
    }
    let directive_arguments = directive_arguments.trim();
    if !directive_arguments.is_empty() {
        directive.with_attr("directive_arguments", directive_arguments);
    }
    stream.consume(); // consume Newline

    if stream.kind_at_cursor() != TK::Indent {
        return Ok(directive);
    }
    directive.with_attr("indent", stream.tokens()[stream.cursor()].lexeme.len());

    if stream.kind_peek_relative(1) == TK::Field {
        stream.consume(); // Skip the shared Indent token; parse_block consumes it otherwise.
        let options = parse_field_list(stream)?;
        directive.push_child(options);
    }

    if stream.kind_at_cursor() != TK::Dedent && stream.kind_at_cursor() != TK::EoF {
        let content = parse_block(stream)?;
        directive.push_child(content);
    }

    Ok(directive)
}

#[cfg(test)]
mod tests {
    use super::parse_directive;
    use crate::lexer::tokenize;
    use crate::token_stream::TokenStream;

    #[test]
    fn parse_substitution_directive_marker() {
        let mut stream = TokenStream::new(tokenize(".. | name | replace:: target\n"));

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
