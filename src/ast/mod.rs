// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod location;
pub mod ast_node;

use crate::ast::ast_node::*;
use crate::value::scalar::*;
use crate::scope::*;
use crate::value::*;
use crate::scope::unit::*;
use crate::operator::*;
use crate::error::*;
use crate::error::collector::*;
use crate::scope::output::*;
use super::ast::location::*;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    Integer(i64),
    Float(f64),
    String(String),
    Unit(String, f64),
    Bool(bool),
    Empty,
    Failed,
}
impl AstNode for LiteralValue {
    type Output = Value;
    fn rev_location(&mut self, _block: usize, _lines_index: &[usize]) {}
    fn evaluate(&self, _scope: &mut Scope, errors: &mut ErrorCollector, _output: &mut OutputCollector) -> Value {
        match self {
            LiteralValue::Integer(integer)  => Value::Scalar(Scalar{ value: *integer as f64, unit: NO_DIMENSION }),
            LiteralValue::Float(float)      => Value::Scalar(Scalar{ value: *float, unit: NO_DIMENSION }),
            LiteralValue::String(_string) => {
                errors.raise(UnsupportedError{functionality: "string"});
                Value::Failed
            },
            LiteralValue::Bool(_bool)       => {
                errors.raise(UnsupportedError{functionality: "bool"});
                Value::Failed
            },
            LiteralValue::Empty                   => Value::Empty,
            LiteralValue::Failed                  => Value::Failed,
            LiteralValue::Unit(label, exponent) => {
                if let Some(unit) = UNIT_DICTIONARY.get(label) {
                    let unit = unit.powf(*exponent);
                    Value::Scalar(Scalar{value: unit.factor, unit: unit.dimension})
                } else {
                    errors.raise(UnfoundUnitError{unit_name: label.to_string()});
                    Value::Failed
                }
            },
        }
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
        op:   Spanned<Leaf<Operator>>,
        args: Vec<Spanned<Expression>>,
    },
    Array           {
        arr: Spanned<Vec<Spanned<Vec<Spanned<Expression>>>>>,
    },
    Assignment      {
        op:    Spanned<Leaf<AssignmentOperator>>,
        ident: Spanned<Leaf<Identifier>>,
        value: Box<Spanned<Expression>>,
    },
    Shown          {
        expr: Box<Spanned<Expression>>,
        op:   Spanned<()>,
        divider: Option<Box<Spanned<Expression>>>,
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
            Expression::Array { arr } => {
                arr.rev_location(block, lines_index);
            }
            Expression::Assignment { op, ident, value } => {
                op.rev_location(block, lines_index);
                ident.rev_location(block, lines_index);
                value.rev_location(block, lines_index);
            }
            Expression::Shown { expr, op, divider } => {
                expr.rev_location(block, lines_index);
                op.rev_location(block, lines_index);
                if let Some(divider) = divider {
                    divider.rev_location(block, lines_index);
                }
            }
        }
    }
    
    fn evaluate(&self, scope: &mut Scope, errors: &mut ErrorCollector, output: &mut OutputCollector) -> Value {
        match self {

            // Leaf
            Expression::Literal(value) => value.evaluate(scope, errors, output),
            Expression::Identifier(identifier) => { 
                let identifier = identifier.evaluate(scope, errors, output);
                scope.get_value(&identifier, errors) 
            },

            // Non-leaf
            Expression::Operation { op, args } => {
                let op = op.evaluate(scope, errors, output);
                let args = args.evaluate(scope, errors, output);
                OPERATOR_TABLE.compute(&op, &args, errors)
            }
            Expression::Array { arr } => {
                /* TODO */
                errors.raise(UnsupportedError{functionality: "array"});
                Value::Failed
            }
            Expression::Assignment { op, ident, value } => {
                let value = value.evaluate(scope, errors, output);
                let ident = ident.evaluate(scope, errors, output);
                let op = op.evaluate(scope, errors, output);
                scope.assign_value(&ident, value, &op, errors);
                Value::Empty
            }
            Expression::Shown { expr, op, divider } => {
                let value = expr.evaluate(scope, errors, output);
                let divider = divider.as_ref().map(|divider| divider.evaluate(scope, errors, output));
                let divider = divider.unwrap_or(Value::Scalar(Scalar{value: 1.0, unit: NO_DIMENSION}));
                let result = OPERATOR_TABLE.compute(&Operator::Shown, &[value.clone(), divider], errors);
                output.add(op.loc_range.end.clone(), format!("{}", result));
                value
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
            (Expression::Array { arr: a }, Expression::Array { arr: b }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:array", prefix).as_str(), &a, &b));
                result
            }
            (Expression::Assignment { op: a, ident: a_ident, value: a_value }, Expression::Assignment { op: b, ident: b_ident, value: b_value }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:op", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:ident", prefix).as_str(), &a_ident, &b_ident));
                result.extend(Spanned::difference(format!("{}:value", prefix).as_str(), &a_value, &b_value));
                result
            }
            (Expression::Shown { expr: a, op: a_op, divider: a_divider }, Expression::Shown { expr: b, op: b_op, divider: b_divider }) => {
                let mut result = Vec::new();
                result.extend(Spanned::difference(format!("{}:expr", prefix).as_str(), &a, &b));
                result.extend(Spanned::difference(format!("{}:op", prefix).as_str(), &a_op, &b_op));
                match (a_divider, b_divider) {
                    (Some(a_divider), Some(b_divider)) => {
                        result.extend(Spanned::difference(format!("{}:divider", prefix).as_str(), &a_divider, &b_divider));
                    }
                    (Some(_), None) => {
                        result.push(format!("{}   - Divider mismatch: {:?} != {:?}", prefix, a_divider, b_divider));
                    }
                    (None, Some(_)) => {
                        result.push(format!("{}   - Divider mismatch: {:?} != {:?}", prefix, a_divider, b_divider));
                    }
                    (None, None) => {}
                }
                result
            }
            _ => vec![format!("{}   - Type mismatch: {:?} != {:?}", prefix, a, b)],
        }
    }
}