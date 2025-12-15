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
        operator:            Spanned<Leaf<Operator>>,
        arguments:           Vec<Spanned<Expression>>,
    },
    Call            {
        identifier:          Spanned<Leaf<Identifier>>,
        arguments:           Vec<Spanned<Expression>>,
    },
    Array           {
        array:               Spanned<Vec<Spanned<Vec<Spanned<Expression>>>>>,
        constructor:         Spanned<Leaf<Operator>>,
    },
    Assignment      {
        identifier:          Spanned<Leaf<Identifier>>,
        assignment_operator: Spanned<Leaf<AssignmentOperator>>,
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
            Expression::Operation { operator, arguments } => {
                operator.rev_location(block, lines_index);
                arguments.rev_location(block, lines_index);
            }
            Expression::Call { identifier, arguments } => {
                identifier.rev_location(block, lines_index);
                arguments.rev_location(block, lines_index);
            }
            Expression::Array { array, constructor } => {
                array.rev_location(block, lines_index);
                constructor.rev_location(block, lines_index);
            }
            Expression::Assignment { identifier, assignment_operator, value } => {
                identifier.rev_location(block, lines_index);
                assignment_operator.rev_location(block, lines_index);
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
            Expression::Operation { operator, arguments } => {
                let operator = operator.evaluate(scope)?;
                let arguments = arguments.evaluate(scope)?;
                operator.compute(&arguments)
            }
            Expression::Call { identifier, arguments } => {
                /*let arguments = arguments.evaluate(scope)?;
                let identifier = identifier.evaluate(scope)?;
                let func = scope.get_function(identifier)?;
                func.call(arguments)*/
                UnsupportedError{functionality: "function call not implemented"}.raise()
            }
            Expression::Array { array, constructor } => {
                /*let array = array.evaluate(scope)?;
                let constructor = constructor.evaluate(scope)?;
                constructor.compute(&array)*/
                UnsupportedError{functionality: "array not implemented"}.raise()
            }
            Expression::Assignment { identifier, assignment_operator, value } => {
                let value = value.evaluate(scope)?;
                let identifier = identifier.evaluate(scope)?;
                let assignment_operator = assignment_operator.evaluate(scope)?;
                scope.assign_value(&identifier, value, &assignment_operator)?;
                Ok(Value::Scalar(Box::new((Scalar::Empty, DEFAULT_UNIT))))
            }
        }
    }

    #[cfg(test)]
    fn difference(prefix: &str, a: &Self, b: &Self) -> Vec<String> {
        match (a, b) {
            (Expression::Literal(a), Expression::Literal(b)) => Spanned::difference(prefix, &a, &b),
            (Expression::Identifier(a), Expression::Identifier(b)) => Spanned::difference(prefix, &a, &b),
            (Expression::Operation { operator: a, arguments: a_arguments }, Expression::Operation { operator: b, arguments: b_arguments }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:operator", prefix).as_str(), &a, &b));
                result.extend(Vec::difference(format!("{}:arguments", prefix).as_str(), &a_arguments, &b_arguments));
                result
            }
            (Expression::Call { identifier: a, arguments: a_arguments }, Expression::Call { identifier: b, arguments: b_arguments }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:identifier", prefix).as_str(), &a, &b));
                result.extend(Vec::difference(format!("{}:arguments", prefix).as_str(), &a_arguments, &b_arguments));
                result
            }
            (Expression::Array { array: a, constructor: a_constructor }, Expression::Array { array: b, constructor: b_constructor }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:array", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:constructor", prefix).as_str(), &a, &b));
                result
            }
            (Expression::Assignment { identifier: a, assignment_operator: a_assignment_operator, value: a_value }, Expression::Assignment { identifier: b, assignment_operator: b_assignment_operator, value: b_value }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:identifier", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:assignment_operator", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:value", prefix).as_str(), &a, &b));
                result
            }
            _ => vec![format!("{}   - Type mismatch: {:?} != {:?}", prefix, a, b)],
        }
    }
}