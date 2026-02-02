// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use super::super::value::*;
use super::super::scope::*;
use super::super::error::*;
use super::super::error::collector::*;
use super::super::scope::output::*;
use super::super::unit::unit::*;
use super::super::unit::dimension::*;
use super::super::value::scalar::*;

use super::ast_node::*;

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