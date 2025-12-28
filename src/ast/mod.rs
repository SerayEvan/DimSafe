// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod location;
pub mod ast_node;

use crate::ast::ast_node::*;
use crate::scope::*;
use crate::scope::value::*;
use crate::scope::scalar::*;
use crate::scope::unit::*;
use crate::scope::operator::*;
use crate::error::*;

use super::ast::location::*;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Unit(String, f32),
    Bool(bool),
    Empty,
    Unspecified,
    Unimplemented,
    Invalid,
}
impl AstNode for LiteralValue {
    type Output = Value;
    fn rev_location(&mut self, _block: usize, _lines_index: &[usize]) {}
    fn evaluate(&self, _scope: &mut Scope) -> Result<Value, Error> {
        Ok(match self {
            LiteralValue::Integer(integer)  => Value::Scalar(Box::new((Scalar::Integer(Integer { value: *integer }), DEFAULT_UNIT))),
            LiteralValue::Float(float)      => Value::Scalar(Box::new((Scalar::Float(Float { value: *float }), DEFAULT_UNIT))),
            LiteralValue::String(string) => Value::String(Box::new(string.clone())),
            LiteralValue::Bool(bool)       => Value::Scalar(Box::new((Scalar::Boolean(Boolean { value: *bool }), DEFAULT_UNIT))),
            LiteralValue::Empty                   => Value::Scalar(Box::new((Scalar::Empty, DEFAULT_UNIT))),
            LiteralValue::Unspecified             => Value::Scalar(Box::new((Scalar::Unspecified, DEFAULT_UNIT))),
            LiteralValue::Unimplemented           => Value::Scalar(Box::new((Scalar::Unimplemented, DEFAULT_UNIT))),
            LiteralValue::Invalid                 => Value::Scalar(Box::new((Scalar::Invalid, DEFAULT_UNIT))),
            LiteralValue::Unit(unit, exponent) => {
                let identifier = Identifier { text: vec![unit.clone()] };
                let unit = _scope.get_unit(&identifier)?.powf(*exponent);
                Value::Scalar(Box::new((Scalar::Integer(Integer { value: 1 }), unit)))
            },
        })
    }
    
    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        if a != b {
            return vec![format!("{}   - Value mismatch: {:?} != {:?}", prefix, a, b)];
        }
        vec![]
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {

    // Leaf
    Literal(Spanned<LiteralValue>),
    Identifier(Spanned<Leaf<Identifier>>),

    // Non-leaf
    Operation       {
        op:            Spanned<Leaf<Operator>>,
        args:           Vec<Spanned<Expression>>,
    },
    Call            {
        ident:          Spanned<Leaf<Identifier>>,
        args:           Vec<Spanned<Expression>>,
    },
    Array           {
        arr:               Spanned<Vec<Spanned<Vec<Spanned<Expression>>>>>,
        op:         Spanned<Leaf<Operator>>,
    },
    Assignment      {
        op: Spanned<Leaf<AssignmentOperator>>,
        ident:          Spanned<Leaf<Identifier>>,
        value:               Box<Spanned<Expression>>,
    },
}

impl AstNode for Expression {
    type Output = Value;

    fn rev_location(&mut self, block: usize, lines_index: &[usize]) {
        match self {

            // Leaf
            Expression::Literal(value) => value.rev_location(block, lines_index),
            Expression::Identifier(identifier) => identifier.rev_location(block, lines_index),

            // Non-leaf
            Expression::Operation { op, args } => {
                op.rev_location(block, lines_index);
                args.rev_location(block, lines_index);
            }
            Expression::Call { ident, args } => {
                ident.rev_location(block, lines_index);
                args.rev_location(block, lines_index);
            }
            Expression::Array { arr, op } => {
                arr.rev_location(block, lines_index);
                op.rev_location(block, lines_index);
            }
            Expression::Assignment { op, ident, value } => {
                op.rev_location(block, lines_index);
                ident.rev_location(block, lines_index);
                value.rev_location(block, lines_index);
            }
        }
    }
    
    fn evaluate(&self, scope: &mut Scope,) -> Result<Value, Error> {
        match self {

            // Leaf
            Expression::Literal(value) => value.evaluate(scope),
            Expression::Identifier(identifier) => { 
                let identifier = identifier.evaluate(scope)?;
                scope.get_value(&identifier) 
            },

            // Non-leaf
            Expression::Operation { op, args } => {
                let op = op.evaluate(scope)?;
                let args = args.evaluate(scope)?;
                op.compute(&args)
            }
            Expression::Call { ident, args } => {
                /*let arguments = arguments.evaluate(scope)?;
                let identifier = identifier.evaluate(scope)?;
                let func = scope.get_function(identifier)?;
                func.call(arguments)*/
                UnsupportedError{functionality: "function call not implemented"}.raise()
            }
            Expression::Array { arr, op } => {
                /*let array = array.evaluate(scope)?;
                let op = op.evaluate(scope)?;
                op.compute(&arr)*/
                UnsupportedError{functionality: "array not implemented"}.raise()
            }
            Expression::Assignment { op, ident, value } => {
                let value = value.evaluate(scope)?;
                let ident = ident.evaluate(scope)?;
                let op = op.evaluate(scope)?;
                scope.assign_value(&ident, value, &op)?;
                Ok(Value::Scalar(Box::new((Scalar::Empty, DEFAULT_UNIT))))
            }
        }
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        match (a, b) {
            (Expression::Literal(a), Expression::Literal(b)) => Spanned::difference(prefix, &a, &b),
            (Expression::Identifier(a), Expression::Identifier(b)) => Spanned::difference(prefix, &a, &b),
            (Expression::Operation { op: a, args: a_args }, Expression::Operation { op: b, args: b_args }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:operator", prefix).as_str(), &a, &b));
                result.extend(Vec::difference(format!("{}:arguments", prefix).as_str(), &a_args, &b_args));
                result
            }
            (Expression::Call { ident: a, args: a_args }, Expression::Call { ident: b, args: b_args }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:identifier", prefix).as_str(), &a, &b));
                result.extend(Vec::difference(format!("{}:arguments", prefix).as_str(), &a_args, &b_args));
                result
            }
            (Expression::Array { arr: a, op: a_op }, Expression::Array { arr: b, op: b_op }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:array", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:op", prefix).as_str(), &a_op, &b_op));
                result
            }
            (Expression::Assignment { op: a, ident: a_ident, value: a_value }, Expression::Assignment { op: b, ident: b_ident, value: b_value }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:op", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:ident", prefix).as_str(), &a_ident, &b_ident));
                result.extend(Spanned::difference(format!("{}:value", prefix).as_str(), &a, &b));
                result
            }
            _ => vec![format!("{}   - Type mismatch: {:?} != {:?}", prefix, a, b)],
        }
    }
}