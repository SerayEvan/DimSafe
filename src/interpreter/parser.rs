// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use log::info;
use lalrpop_util::ParseError;

use super::ast::expression::*;
use super::ast::location::*;
use super::ast::ast_node::*;
use super::lexer::*;

use super::grammar::*;

fn get_lines_pos(input: &str) -> Vec<usize> {
    let mut lines_index = vec![0];
    for (i, c) in input.chars().enumerate() {
        if c == '\n' {
            lines_index.push(i + 1);
        }
    }
    lines_index
}

fn get_error_location(error: &ParseError<usize, Token, (usize, LexicalError, usize)>) -> ErrorLocation {
    match error {
        ParseError::InvalidToken { location, .. } => {
            ErrorLocation { loc_range: RangeIndex::new(*location, *location) }
        }
        ParseError::UnrecognizedEof { location, .. } => {
            ErrorLocation { loc_range: RangeIndex::new(*location, *location) }
        }
        ParseError::UnrecognizedToken { token, expected } => {
            ErrorLocation { loc_range: RangeIndex::new(token.0, token.2) }
        }
        ParseError::ExtraToken { token } => {
            ErrorLocation { loc_range: RangeIndex::new(token.0, token.2) }
        }
        ParseError::User  { error } => {
            ErrorLocation { loc_range: RangeIndex::new(error.0, error.2) }
        }
    }
}

#[derive(Debug)]
pub struct ErrorLocation {
    pub loc_range: RangeIndex,
}

pub fn parse_program(input: &str) -> Result<Vec<Spanned<Expression>>, ErrorLocation> {

    info!("Parsing program: {}", input);

    // get lines index
    let lines_pos = get_lines_pos(input);

    info!("Lines positions: {:?}", lines_pos);

    // lexer
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<_>>();

    // show tokens
    info!("{:?}", tokens);

    // parse program
    let program = ProgramParser::new().parse(tokens);

    // reverse location
    match program {
        Ok(mut program) => {
            program.rev_location(0, &lines_pos);
            Ok(program)
        }
        Err(e) => {
            Err(get_error_location(&e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::interpreter::operator::*;
    use crate::interpreter::ast::literal_value::*;

    #[test]
    fn test_get_lines_pos() {
        let lines_pos = get_lines_pos("1+2*3 .m");
        assert_eq!(lines_pos, vec![0]);

        let lines_pos = get_lines_pos("1+2*3 .m\n4+5*6 .kg");
        assert_eq!(lines_pos, vec![0, 9]);
    }

    #[test]
    fn test_parser() {
        let program = parse_program("(1 .m + 2)*3 .kg").unwrap();
        assert_eq!(program.len(), 1);
        
        let expected = 
        Spanned::new(RangeIndex::new(0, 16), Expression::Operation{
                op: Spanned::new(RangeIndex::new(10, 11), Leaf::from(Operator::ArithmeticMul)),
                args: vec![
                    Spanned::new(RangeIndex::new(0, 10), Expression::Operation{
                        op: Spanned::new(RangeIndex::new(6, 7), Leaf::from(Operator::ArithmeticAdd)),
                        args: vec![
                            Spanned::new(RangeIndex::new(1, 5), Expression::Operation{
                                op: Spanned::new(RangeIndex::new(3, 3), Leaf::from(Operator::Juxtaposition)),
                                args: vec![
                                    Spanned::new(RangeIndex::new(1, 2), Expression::Literal(Spanned::new(RangeIndex::new(1, 2), LiteralValue::Integer(1)))),
                                    Spanned::new(RangeIndex::new(3, 5), Expression::Literal(Spanned::new(RangeIndex::new(3, 5), LiteralValue::Unit("m".to_string(), 1.0)))),
                                ],
                            }),
                            Spanned::new(RangeIndex::new(8, 9), Expression::Literal(Spanned::new(RangeIndex::new(8, 9), LiteralValue::Integer(2)))),
                        ],
                    }),
                    Spanned::new(RangeIndex::new(11, 16), Expression::Operation{
                        op: Spanned::new(RangeIndex::new(13, 13), Leaf::from(Operator::Juxtaposition)),
                        args: vec![
                            Spanned::new(RangeIndex::new(11, 12), Expression::Literal(Spanned::new(RangeIndex::new(11, 12), LiteralValue::Integer(3)))),
                            Spanned::new(RangeIndex::new(13, 16), Expression::Literal(Spanned::new(RangeIndex::new(13, 16), LiteralValue::Unit("kg".to_string(), 1.0)))),
                        ],
                    }),
                ],
            });
        let difference = Spanned::difference("", &program[0], &expected);

        for diff in &difference {
            println!("{}", diff);
        }
        
        assert!(difference.is_empty());
    }
}