// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::unit::*;

pub trait ValueTrait: Sized {
    fn type_id() -> TypeId;
    fn try_from_value(value: Value) -> Option<Self>;
    fn into_value(self) -> Value;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub value: f64,
    pub unit: UnitDimension,
}

impl ValueTrait for Scalar {
    fn type_id() -> TypeId { TypeId::Scalar }
    fn try_from_value(value: Value) -> Option<Self> {
        match value {
            Value::Scalar(scalar) => Some(scalar),
            _ => None
        }
    }
    fn into_value(self) -> Value {
        Value::Scalar(self)
    }
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