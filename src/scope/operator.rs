// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::Debug;
use std::collections::HashMap;

use crate::error::*;
use super::value::*;
use lazy_static::lazy_static;

#[derive(Debug, Clone, PartialEq)]
struct OperatorImplementation {
    inputs_type: Vec<ValueCategory>,
    output_type: Option<ValueCategory>,
    symmetric: bool,
    pub implementation: fn(&[Value]) -> Result<Value, Error>,
}

impl OperatorImplementation {
    fn matches(&self, inputs: &[Value]) -> bool {
        let inputs_type: Vec<ValueCategory> = inputs.iter().map(|input| input.get_type()).collect();
        if self.symmetric {
            return self.inputs_type.iter().eq(inputs_type.iter()) || self.inputs_type.iter().rev().eq(inputs_type.iter());
        }
        return self.inputs_type.iter().eq(inputs_type.iter());
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Juxtaposition,
    ArithmeticAdd,
    ArithmeticSub,
    ArithmeticMul,
    ArithmeticDiv,
    ArithmeticPow,
    VectorDot,
    VectorDet,
    VectorCross,
    BooleanNot,
    BooleanAnd,
    BooleanOr,
    BooleanXor,
    ComparatorEqual,
    ComparatorNotEqual,
    ComparatorGreaterOrEqual,
    ComparatorLessOrEqual,
    ComparatorGreaterThan,
    ComparatorLessThan,
    ComparatorIn,
    ComparatorHas,
    ComparatorIs,
    ConstructorTable,
    ConstructorList,
    ConstructorMatrix,
    ConstructorVector,
}

lazy_static! {
    static ref implementationsTable: HashMap<Operator, Vec<OperatorImplementation>> = HashMap::new();
}

impl Operator {
    pub fn compute(&self, inputs: &[Value]) -> Result<Value, Error> {
        for implementation in implementationsTable.get(self).unwrap() {
            if implementation.matches(inputs) {
                return (implementation.implementation)(inputs);
            }
        }
        UnsupportedError{functionality: "operator implementation for given inputs not found"}.raise()
    }
}

/*pub static ADD_OPERATOR: Operator = Operator::new("add");
pub static SUB_OPERATOR: Operator = Operator::new("sub");
pub static MUL_OPERATOR: Operator = Operator::new("mul");
pub static DIV_OPERATOR: Operator = Operator::new("div");
pub static POW_OPERATOR: Operator = Operator::new("pow");
pub static DOT_OPERATOR: Operator = Operator::new("dot");
pub static DET_OPERATOR: Operator = Operator::new("det");
pub static CROSS_OPERATOR: Operator = Operator::new("cross");
pub static NOT_OPERATOR: Operator = Operator::new("not");
pub static AND_OPERATOR: Operator = Operator::new("and");
pub static OR_OPERATOR: Operator = Operator::new("or");
pub static XOR_OPERATOR: Operator = Operator::new("xor");
pub static EQUAL_OPERATOR: Operator = Operator::new("equal");
pub static NOT_EQUAL_OPERATOR: Operator = Operator::new("not_equal");
pub static GREATER_OR_EQUAL_OPERATOR: Operator = Operator::new("greater_or_equal");
pub static LESS_OR_EQUAL_OPERATOR: Operator = Operator::new("less_or_equal");
pub static GREATER_THAN_OPERATOR: Operator = Operator::new("greater_than");
pub static LESS_THAN_OPERATOR: Operator = Operator::new("less_than");
pub static IN_OPERATOR: Operator = Operator::new("in");
pub static HAS_OPERATOR: Operator = Operator::new("has");
pub static IS_OPERATOR: Operator = Operator::new("is");

pub static TABLE_CONSTRUCTOR: Operator = Operator::new("Table");
pub static LIST_CONSTRUCTOR: Operator = Operator::new("List");
pub static MATRIX_CONSTRUCTOR: Operator = Operator::new("Matrix");
pub static VECTOR_CONSTRUCTOR: Operator = Operator::new("Vector");*/