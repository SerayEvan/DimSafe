// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod scalar;

use super::value::scalar::*;
use crate::operator::table::*;
use lazy_static::lazy_static;
use std::fmt;
use std::fmt::Display;

pub trait ValueTrait: Sized {
    fn type_id() -> TypeId;
    fn try_from_value(value: &Value) -> Option<&Self>;
    fn into_value(self) -> Value;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(Scalar),
    Failed,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeId {
    Scalar,
    Failed,
    Empty,
}

impl Value {
    pub fn get_type(&self) -> TypeId {
        match self {
            Value::Scalar(_) => TypeId::Scalar,
            Value::Failed => TypeId::Failed,
            Value::Empty => TypeId::Empty,
        }
    }

    pub fn match_category(&self, value_category: TypeId) -> bool {
        match self {
            Value::Scalar(_) => value_category == TypeId::Scalar,
            Value::Failed => value_category == TypeId::Failed,
            Value::Empty => value_category == TypeId::Empty,
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Scalar(scalar) => write!(f, "{}", scalar),
            Value::Failed => write!(f, "Failed"),
            Value::Empty => write!(f, "Empty"),
        }
    }
}

fn get_operator_table() -> OperatorTable {
    let mut operator_table = OperatorTable::new();
    scalar_operator(&mut operator_table);
    operator_table
}

lazy_static! {
    pub static ref OPERATOR_TABLE: OperatorTable = get_operator_table();
}
