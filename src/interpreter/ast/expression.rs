// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use super::super::value::scalar::*;
use super::super::scope::*;
use super::super::value::*;
use super::super::unit::dimension::*;
use super::super::operator::*;
use super::super::error::*;
use super::super::error::collector::*;
use super::super::scope::output::*;

use super::location::*;
use super::ast_node::*;
use super::literal_value::*;

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
            Expression::Array { arr: _ } => {
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