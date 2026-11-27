// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT
use rstu_ast::NodeRefExt;

use crate::parser_errors::ParserError;
use crate::token::TokenKind as TK;
use crate::token_stream::TokenStream;

pub(crate) mod block;
pub(crate) mod block_quote;
pub(crate) mod comments;
pub(crate) mod definition_list;
pub(crate) mod directives;
pub(crate) mod doctest_block;
pub(crate) mod footnote_citation;
pub(crate) mod hyperlink_reference;
pub(crate) mod inline_markup;
pub(crate) mod line_block;
pub(crate) mod lists;
pub(crate) mod literal_block;
pub(crate) mod paragraph;
pub(crate) mod section;
pub(crate) mod targets;
pub(crate) mod transition;
use regex::Regex;
use std::sync::LazyLock;

static REFERENCE_ESCAPE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\\(?s:(.))|\\$").unwrap());

pub(crate) fn with_hanging_indent<F>(
    stream: &mut TokenStream,
    parser: F,
) -> Result<rstu_ast::NodeRef, ParserError>
where
    F: Fn(&mut TokenStream) -> Result<rstu_ast::NodeRef, ParserError>,
{
    let index_next_line: usize = stream.find_end_of_line() + 1;
    let indent = match stream.token_at(index_next_line).kind {
        // TODO: use token_at_nextline
        TK::Indent => stream.take_at(index_next_line).len(),
        _ => 0,
    };

    let node = parser(stream)?;

    if stream.token_at_cursor().kind == TK::NewLine {
        node.with_attr("finally", "\n");
        stream.consume(); // TODO: This should be part of the inner function. Option: Pass a second post-processing function?
    }

    if indent > 0 {
        node.with_attr("hanging_indent", indent);

        let dedent = stream.token_at_cursor();
        if dedent.kind != TK::Dedent {
            return Err(ParserError::UnexpectedToken {
                expected: "Dedent".to_string(),
                found: format!("{:?}", dedent.kind),
                position: stream.position_at_cursor(),
            });
        } else if dedent.len() > indent {
            stream.update_at_cursor(" ".repeat(dedent.len() - indent));
        } else if dedent.len() < indent {
            return Err(ParserError::DedentToSmall {
                expected: dedent.len(),
                found: indent,
                position: stream.position_at_cursor(),
            });
        } else {
            stream.consume();
        }
    }

    Ok(node)
}

pub(crate) fn normalize_name(reference: &str) -> String {
    REFERENCE_ESCAPE
        .replace_all(reference, "$1")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
