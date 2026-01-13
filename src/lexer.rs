// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt;

use logos::{Logos, SpannedIter};

use crate::ast::*;
use crate::editor::stylization::*;
use super::operator::*;
use super::scope::*;

fn parse_integer(input: &str, base: u32) -> LiteralValue {
    // TODO: manage overflow
    LiteralValue::Integer(
        i64::from_str_radix(input, base).unwrap()
    )
}

fn parse_float(input: &str) -> LiteralValue {
    // TODO: manage overflow
    LiteralValue::Float(
        input.parse::<f64>().unwrap()
    )
}

fn parse_string(input: &str) -> LiteralValue {
    let mut result = String::new();
    let mut chars = input.chars().peekable(); // Utilise un itérateur avec un aperçu

    while let Some(c) = chars.next() {
        match c {
            '"' => continue, // Ignore les guillemets de début et de fin
            '\\' => {
                // Traitement des séquences d'échappement
                if let Some(next) = chars.next() {
                    match next {
                        'n' => result.push('\n'),
                        't' => result.push('\t'),
                        'r' => result.push('\r'),
                        '\\' => result.push('\\'),
                        '\"' => result.push('\"'),
                        'x' => {
                            // Lire deux caractères hexadécimaux
                            let hex: String = chars.by_ref().take(2).collect();
                            match u8::from_str_radix(&hex, 16) {
                                Ok(value) => result.push(value as char),
                                Err(_) => panic!("Invalid hexadecimal escape sequence: \\x{}", hex),
                            }
                        },
                        'u' => {
                            // Lire quatre caractères hexadécimaux
                            let hex: String = chars.by_ref().take(4).collect();
                            match u32::from_str_radix(&hex, 16) {
                                Ok(value) => match char::from_u32(value) {
                                    Some(ch) => result.push(ch),
                                    None => panic!("Invalid Unicode scalar value: \\u{}", hex),
                                },
                                Err(_) => panic!("Invalid hexadecimal escape sequence: \\u{}", hex),
                            }
                        },
                        'U' => {
                            // Lire huit caractères hexadécimaux
                            let hex: String = chars.by_ref().take(8).collect();
                            match u32::from_str_radix(&hex, 16) {
                                Ok(value) => match char::from_u32(value) {
                                    Some(ch) => result.push(ch),
                                    None => panic!("Invalid Unicode scalar value: \\U{}", hex),
                                },
                                Err(_) => panic!("Invalid hexadecimal escape sequence: \\U{}", hex),
                            }
                        },
                        _ => result.push(next), // Pour tout autre caractère échappé, l'ajouter tel quel
                    }
                }
            },
            _ => result.push(c), // Ajouter le caractère normal
        }
    }

    LiteralValue::String(result)
}

pub fn parse_unit(input: &str) -> LiteralValue {

    // split string in 2 parts : [unit] [exponent with optional + or -]
    let mut split_idx = 0;
    for c in input.chars() {
        if c == '+' || c == '-' || (c >= '0' && c <= '9') {
            break;
        }
        split_idx += c.len_utf8();
    }

    let unit = &input[..split_idx].to_string();
    
    let exponent = if split_idx == input.len() { 
        1.0
    } else {
        input[split_idx..].parse::<i32>().unwrap_or(1) as f64
    };

    LiteralValue::Unit(unit.to_string(), exponent)
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructuralToken {
    LParenthesis,
    RParenthesis,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Arraow,
    Comma,
    Semicolon,
    Dot,
    Dollar,
    Percent,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexicalError)] // Ignore this regex pattern between tokens
pub enum Token {

    // Comentary (moved to RawError as skip)

    // Structural tokens
    #[token("(", |_| StructuralToken::LParenthesis)]
    #[token(")", |_| StructuralToken::RParenthesis)]
    #[token("[", |_| StructuralToken::LBracket)]
    #[token("]", |_| StructuralToken::RBracket)]
    #[token("{", |_| StructuralToken::LBrace)]
    #[token("}", |_| StructuralToken::RBrace)]
    #[token("->", |_| StructuralToken::Arraow)]
    #[token(",", |_| StructuralToken::Comma)]
    #[token(";", |_| StructuralToken::Semicolon)]
    #[token(".", |_| StructuralToken::Dot)]
    #[token("$", |_| StructuralToken::Dollar)]
    #[token("%", |_| StructuralToken::Percent)]
    StructuralToken(StructuralToken),

    // Assignment operator
    #[token(":", |_| AssignmentOperator::Define)]
    #[token(":=", |_| AssignmentOperator::Reassign)]
    #[token(":++", |_| AssignmentOperator::Push)]
    AssignmentOperator(AssignmentOperator),

    // literals value

    // integer
    #[regex(r"[0-9]+", |lex| parse_integer(lex.slice(), 10))]
    #[regex(r"0x[0-9a-fA-F]+", |lex| parse_integer(&lex.slice()[2..], 16))]
    #[regex(r"0b[01]+", |lex| parse_integer(&lex.slice()[2..], 2))]
    #[regex(r"0o[0-7]+", |lex| parse_integer(&lex.slice()[2..], 8))]
    // float
    #[regex(r#"[0-9]*\.[0-9]+"#, |lex| parse_float(lex.slice()))]
    #[regex(r#"[0-9]*\.[0-9]+e-?[0-9]*"#, |lex| parse_float(lex.slice()))]
    LiteralNumerical(LiteralValue),

    // boolean
    #[token("true", |_| LiteralValue::Bool(true))]
    #[token("false", |_| LiteralValue::Bool(false))]
    // meta
    #[token("Failed", |_| LiteralValue::Failed)]
    #[token("Empty", |_| LiteralValue::Empty)]
    LiteralKeyword(LiteralValue),

    // string
    #[regex(r#""([^"\\]|\\.)*""#, |lex| parse_string(lex.slice()))]
    #[regex(r#"'([^'\\]|\\.)*'"#, |lex| parse_string(lex.slice()))]
    LiteralString(LiteralValue),

    // unit
    #[regex(r#"\.\p{XID_Start}+[\-\+]?[0-9]*"#, |lex|  parse_unit(&lex.slice()[1..]))]
    LiteralUnit(LiteralValue),

    // Operators
    #[token("+", |_| Operator::ArithmeticAdd)]
    AdditiveOperator(Operator),
    #[token("-", |_| Operator::ArithmeticSub)]
    SubtractiveOperator(Operator),
    #[token("*", |_| Operator::ArithmeticMul)]
    #[token("/", |_| Operator::ArithmeticDiv)]
    MultiplicativeOperator(Operator),
    #[token("^", |_| Operator::ArithmeticPow)]
    PowerOperator(Operator),

    // vector operation
    #[token("dot", |_| Operator::VectorDot)]
    #[token("det", |_| Operator::VectorDet)]
    #[token("cross", |_| Operator::VectorCross)]
    VectorMultiplicativeOperator(Operator),

    // Bolean operation
    #[token("not", |_| Operator::BooleanNot)]
    #[token("and", |_| Operator::BooleanAnd)]
    #[token("or", |_| Operator::BooleanOr)]
    #[token("xor", |_| Operator::BooleanXor)]
    BooleanOperator(Operator),

    // Comparators
    #[token("="  , |_| Operator::ComparatorEqual)]
    #[token("!=" , |_| Operator::ComparatorNotEqual)]
    #[token(">=" , |_| Operator::ComparatorGreaterOrEqual)]
    #[token("<=" , |_| Operator::ComparatorLessOrEqual)]
    #[token(">"  , |_| Operator::ComparatorGreaterThan)]
    #[token("<"  , |_| Operator::ComparatorLessThan)]
    #[token("in" , |_| Operator::ComparatorIn)]
    #[token("has", |_| Operator::ComparatorHas)]
    #[token("is" , |_| Operator::ComparatorIs)]
    ComparatorOperator(Operator),

    // Indentifier
    #[regex(r#"[\p{XID_Start}_]\p{XID_Continue}*"#, |lex| lex.slice().to_string())]
    Identifier(String),

    // Skip
    #[regex(r"[ \r\v\t\n\f]+", logos::skip)]
    #[regex(r#"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/"#, logos::skip)]
    RawError,
}

impl Token {
    pub fn get_balise(&self) -> usize {
        match self {
            Token::LiteralNumerical(_) => LITERAL_NUMERICAL_BALISE,
            Token::LiteralKeyword(_) => LITERAL_KEYWORD_BALISE,
            Token::LiteralString(_) => LITERAL_STRING_BALISE,
            Token::LiteralUnit(_) => LITERAL_UNIT_BALISE,
            Token::Identifier(_) => IDENTIFIER_BALISE,
            Token::StructuralToken(_) => STRUCTURAL_BALISE,
            Token::AssignmentOperator(_) => STRUCTURAL_BALISE,
            Token::AdditiveOperator(_) => OPERATOR_BALISE,
            Token::SubtractiveOperator(_) => OPERATOR_BALISE,
            Token::MultiplicativeOperator(_) => OPERATOR_BALISE,
            Token::PowerOperator(_) => OPERATOR_BALISE,
            Token::VectorMultiplicativeOperator(_) => OPERATOR_BALISE,
            Token::BooleanOperator(_) => OPERATOR_BALISE,
            Token::ComparatorOperator(_) => OPERATOR_BALISE,
            Token::RawError => EMPTY_BALISE,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
    }
}

pub type SpannedToken<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

pub struct Lexer<'input> {
  // instead of an iterator over characters, we have a token iterator
  token_stream: SpannedIter<'input, Token>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        // the Token::lexer() method is provided by the Logos trait
        Self { token_stream: Token::lexer(input).spanned() }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = SpannedToken<Token, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.token_stream
        .next()
        .map(|(token, span)| Ok((span.start, token?, span.end)))
    }
}

impl<'input> Lexer<'input> {
    pub fn stylize(self, stylization: &mut Stylization) {
        for token in self {
            if let Ok((tok_start, tok, tok_end)) = token {
                stylization.insert_balise(tok.get_balise(), (tok_start, tok_end));
            }
        }
    }
}

// test the lexer
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut lexer = Lexer::new("1+2*3 .m");
        assert_eq!(lexer.next(), Some(Ok((0, Token::LiteralNumerical(LiteralValue::Integer(1)), 1))));
        assert_eq!(lexer.next(), Some(Ok((1, Token::AdditiveOperator(Operator::ArithmeticAdd), 2))));
        assert_eq!(lexer.next(), Some(Ok((2, Token::LiteralNumerical(LiteralValue::Integer(2)), 3))));
        assert_eq!(lexer.next(), Some(Ok((3, Token::MultiplicativeOperator(Operator::ArithmeticMul), 4))));
        assert_eq!(lexer.next(), Some(Ok((4, Token::LiteralNumerical(LiteralValue::Integer(3)), 5))));
        assert_eq!(lexer.next(), Some(Ok((6, Token::LiteralUnit(LiteralValue::Unit("m".to_string(), 1.0)), 8))));
    }

    #[test]
    fn test_lexer_with_parenthesis() {
        let mut lexer = Lexer::new("sin(x+y)*3");
        assert_eq!(lexer.next(), Some(Ok((0, Token::Identifier("sin".to_string()), 3))));
        assert_eq!(lexer.next(), Some(Ok((3, Token::StructuralToken(StructuralToken::LParenthesis), 4))));
        assert_eq!(lexer.next(), Some(Ok((4, Token::Identifier("x".to_string()), 5))));
        assert_eq!(lexer.next(), Some(Ok((5, Token::AdditiveOperator(Operator::ArithmeticAdd), 6))));
        assert_eq!(lexer.next(), Some(Ok((6, Token::Identifier("y".to_string()), 7))));
        assert_eq!(lexer.next(), Some(Ok((7, Token::StructuralToken(StructuralToken::RParenthesis), 8))));
        assert_eq!(lexer.next(), Some(Ok((8, Token::MultiplicativeOperator(Operator::ArithmeticMul), 9))));
        assert_eq!(lexer.next(), Some(Ok((9, Token::LiteralNumerical(LiteralValue::Integer(3)), 10))));
    }
}