// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use log::info;
use lalrpop_util::ParseError;

use super::ast::expression::*;
use super::ast::location::*;
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

fn get_error_location(error: &ParseError<usize, Token, (usize, LexicalError, usize)>) -> RangeIndex {
    match error {
        ParseError::InvalidToken { location, .. } => {
            RangeIndex::new(*location, *location)
        }
        ParseError::UnrecognizedEof { location, .. } => {
            RangeIndex::new(*location, *location)
        }
        ParseError::UnrecognizedToken { token, .. } => {
            RangeIndex::new(token.0, token.2)
        }
        ParseError::ExtraToken { token } => {
            RangeIndex::new(token.0, token.2)
        }
        ParseError::User  { error } => {
            RangeIndex::new(error.0, error.2)
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvalidTokenError {
    pub loc_range: Vec<RangeIndex>,
}

impl InvalidTokenError {
    pub fn new() -> Self {
        Self { loc_range: vec![] }
    }
}

pub fn parse_program(input: &str) -> Result<Vec<Instruction>, InvalidTokenError> {

    info!("Parsing program: {}", input);

    // get lines index
    let lines_pos = get_lines_pos(input);

    info!("Lines positions: {:?}", lines_pos);

    // lexer
    let lexer = Lexer::new(input);
    let tokens = lexer.collect::<Vec<_>>();

    // verify all tokens are valid
    let invalid_tokens = tokens.iter()
        .filter_map(|tok| {
            match tok {
                Ok(_) => None,
                Err((start, _, end)) => Some(RangeIndex::new(*start, *end)),
            }
        }).collect::<Vec<RangeIndex>>();

    if !invalid_tokens.is_empty() {
        return Err(InvalidTokenError { loc_range: invalid_tokens });
    }

    // show tokens
    info!("{:?}", tokens);

    // parse program
    let program = ProgramParser::new().parse(tokens);

    // reverse location
    match program {
        Ok(program) => {
            Ok(program)
        }
        Err(e) => {
            Err(InvalidTokenError { loc_range: vec![get_error_location(&e)] })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::super::ast::ast_node::*;
    use super::super::operator::*;
    use super::super::ast::literal_value::*;

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
        Instruction{expr: Spanned::new(RangeIndex::new(0, 16), Expression::Operation{
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
            })};
        let difference = Instruction::difference("", &program[0], &expected);

        for diff in &difference {
            println!("{}", diff);
        }
        
        assert!(difference.is_empty());
    }
}