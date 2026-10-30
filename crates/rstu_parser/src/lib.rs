// SPDX-FileCopyrightText: 2026 Jochen Schmaehling <tostmann1@web.de>
//
// SPDX-License-Identifier: MIT

macro_rules! space {
    ($width:expr) => {
        " ".repeat($width)
    };
}

pub mod lexer;
mod parsers;
use std::cell::RefCell;
use std::rc::Rc;

use parsers::block_quote::parse_block_quote;
use parsers::comments::parse_comment;
use parsers::definition_list::{is_definition_list_item, parse_definition_list};
use parsers::directives::parse_directive;
use parsers::doctest_block::parse_doctest_block;
use parsers::footnote::parse_footnote;
use parsers::line_block::parse_line_block;
use parsers::lists::{
    parse_bullet_list, parse_enumerated_list, parse_field_list, parse_option_list,
};
use parsers::literal_block::parse_literal_block;
use parsers::paragraph::parse_paragraph;
use parsers::section::parse_section_header;
use parsers::transition::parse_transition;

pub mod parser_errors;
pub mod token;
pub mod token_stream;

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::lexer::tokenize;
use crate::parsers::paragraph::parse_paragraph_hanging_indent;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use parser_errors::ParserError;
use token_stream::TokenStream;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, ParserError> {
    let mut stream = tokenize(input);
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut current_parent = doc.clone();

    parse_body_element(&mut stream, &mut current_parent)?;

    doc.prune_leading_blankline();
    Ok(doc)
}

fn parse_body_element(
    stream: &mut TokenStream,
    current_parent: &mut Rc<RefCell<AstNode>>,
) -> Result<(), ParserError> {
    loop {
        match (
            stream.token_at_cursor().kind,
            stream.token_at_nextline().kind,
        ) {
            (TK::BlankLine, TK::Separator) => {
                current_parent.push_blank_lines(stream.consume().len());

                if matches!(stream.token_at_nextline().kind, TK::BlankLine | TK::EoF) {
                    let transition = parse_transition(stream)?;
                    current_parent.push_child(transition);
                }
                // else: do section parsing in next iteration.
            }
            (TK::Separator, _) | (_, TK::Separator) => {
                let section = parse_section_header(stream)?;
                current_parent.push_section_ref(section.clone());
                *current_parent = section;
            }
            (TK::Indent, _) => {
                return Err(ParserError::UnexpectedIndentError {
                    position: stream.position_at_cursor(),
                });
            }
            (TK::EoF, _) => {
                break;
            }
            (TK::BulletListMarker, _) => {
                let bullet_list = parse_bullet_list(stream)?;
                current_parent.push_child(bullet_list);
            }
            (TK::Directive, _) => {
                let directive = parse_directive(stream)?;
                current_parent.push_child(directive);
            }

            (TK::DoctestStart, _) => {
                let doctest_block = parse_doctest_block(stream)?;
                current_parent.push_child(doctest_block);
            }

            (TK::FootNote, _) => {
                let footnote = parse_footnote(stream)?;
                current_parent.push_child(footnote);
            }

            (TK::DoubleDot, _) => {
                let comment = parse_comment(stream)?;
                current_parent.push_child(comment);
            }

            (TK::Field, _) => {
                let field_list = parse_field_list(stream)?;
                current_parent.push_child(field_list);
            }

            (TK::EnumeratedListMarker, _) => {
                let enumerated_list = parse_enumerated_list(stream)?;
                current_parent.push_child(enumerated_list);
            }

            _ if is_definition_list_item(stream) => {
                let definition_list = parse_definition_list(stream)?;
                current_parent.push_child(definition_list);
            }

            (TK::Option, _)
                if matches!(
                    stream.token_at_nextline().kind,
                    TK::BlankLine | TK::Option | TK::Indent | TK::EoF
                ) =>
            {
                let option_list = parse_option_list(stream)?;
                current_parent.push_child(option_list);
            }

            (
                TK::LiteralBlock | TK::LiteralBlockMinimized | TK::LiteralBlockPartiallyMinimized,
                _,
            ) => {
                let literal_block = parse_literal_block(stream)?;
                current_parent.push_child(literal_block);
            }

            (TK::LineBlock, _) => {
                let line_block = parse_line_block(stream, 0)?;
                current_parent.push_child(line_block);
            }

            (TK::BlankLine, _) if stream.token_at_nextline().kind == TK::Indent => {
                let block_quote = parse_block_quote(stream)?;
                current_parent.push_child(block_quote);
            }
            (TK::BlankLine, _) => {
                current_parent.push_blank_lines(stream.consume().len());
            }
            (TK::Attribution, _) => {
                let attribution = AstNode::new_ref(NodeClass::Attribution);
                attribution.with_attr("marker", stream.consume().lexeme);
                let paragraph = parse_paragraph_hanging_indent(stream)?;
                attribution.push_child(paragraph);
                current_parent.push_child(attribution);
            }
            (kind, _) if kind.is(TC::PARAGRAPH_END) => {
                return Err(ParserError::UnexpectedToken {
                    expected: "Some content.".to_owned(),
                    found: format!("{:?}", kind),
                    position: stream.position_at_cursor(),
                });
            }
            _ => {
                let paragraph = parse_paragraph(stream)?;
                current_parent.push_child(paragraph);
            }
        }
    }
    Ok(())
}
