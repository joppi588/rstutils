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

use parsers::comments::parse_comment;
use parsers::directives::parse_directive;
use parsers::list::{parse_bullet_list, parse_enumerated_list, parse_field_list};
use parsers::literal_block::parse_literal_block;
use parsers::paragraph::parse_paragraph;
use parsers::section::parse_section_header;

pub mod parser_errors;
pub mod token;
pub mod token_stream;

use rstu_ast::{AstNode, NodeClass, NodeRef, NodeRefExt};

use crate::lexer::tokenize;
use crate::token::{TokenCategory as TC, TokenKind as TK};
use parser_errors::ParserError;
use token_stream::TokenStream;

// static DEDENT_GRACE: usize = 1;

/// Parser implementation:
/// Lookahead one line -> Decide on element.
pub fn parse(input: &str) -> Result<NodeRef, ParserError> {
    let mut stream = tokenize(input);
    let doc = AstNode::new_ref(NodeClass::Document);
    let mut current_parent = doc.clone();

    loop {
        match (stream.kind_at_cursor(), stream.kind_at_nextline()) {
            (TK::Separator, TK::Indent | TK::Word) | (TK::Word, TK::Separator) => {
                let section = parse_section_header(&mut stream)?;
                current_parent.push_section_ref(section.clone());
                current_parent = section;
            }
            (TK::Directive, _) => {
                let directive = parse_directive(&mut stream)?;
                current_parent.push_child(directive);
            }

            (TK::DoubleDot, _) => {
                let comment = parse_comment(&mut stream)?;
                current_parent.push_child(comment);
            }

            // TODO: Do not simply ignore these
            (TK::Indent, _) | (TK::Dedent, _) => {
                stream.consume();
            }

            (kind, _) if kind.nested_is(TC::BODY_ELEMENTS) => {
                parse_body_elements(&mut stream, &current_parent)?;
            }
            (TK::BlankLine, _) => {
                let token = stream.consume();
                current_parent.push_blank_lines(token.len());
            }

            (TK::EoF, _) => {
                break;
            }
            _ => panic!(
                "Unexpected token combination ({:?},{:?})",
                stream.kind_at_cursor(),
                stream.kind_at_nextline()
            ),
        };
    }

    Ok(doc)
}

fn parse_body_elements(
    stream: &mut TokenStream,
    current_parent: &Rc<RefCell<AstNode>>,
) -> Result<(), ParserError> {
    match stream.kind_at_cursor() {
        TK::BulletListMarker => {
            let bullet_list = parse_bullet_list(stream)?;
            current_parent.push_child(bullet_list);
        }

        TK::Field => {
            let field_list = parse_field_list(stream)?;
            current_parent.push_child(field_list);
        }

        TK::EnumeratedListMarker => {
            let enumerated_list = parse_enumerated_list(stream)?;
            current_parent.push_child(enumerated_list);
        }

        kind if kind.nested_is(TC::PARAGRAPH) => {
            let paragraph = parse_paragraph(stream)?;
            current_parent.push_child(paragraph);
        }
        TK::LiteralBlock | TK::LiteralBlockMinimized | TK::LiteralBlockPartiallyMinimized => {
            let literal_block = parse_literal_block(stream)?;
            current_parent.push_child(literal_block);
        }

        _ => {
            panic!(
                "Token kind {:?} not in {:?}",
                stream.kind_at_cursor(),
                TC::BODY_ELEMENTS
            );
        }
    }

    Ok(())
}
