// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::unit::*;

use super::scalar::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    value: Vec<Scalar>,
    unit: UnitDimension,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: usize,
    columns: usize,
    rows_indentifier: Option<String>,
    rows_unit: Option<UnitDimension>,
    data: Vec<Scalar>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    rows: usize,
    columns: usize,
    rows_indentifier: Option<String>,
    rows_unit: Option<UnitDimension>,
    data: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Scalar(Box<(Scalar, Unit)>),
    Array(Box<Array>),
    Matrix(Box<Matrix>),
    Vector(Box<Vector>),
    String(Box<String>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValueCategory {
    Scalar,
    Array,
    Matrix,
    Vector,
    String,
    Bool,
}

impl Value {
    pub fn get_type(&self) -> ValueCategory {
        match self {
            Value::Scalar(_) => ValueCategory::Scalar,
            Value::Array(_) => ValueCategory::Array,
            Value::Matrix(_) => ValueCategory::Matrix,
            Value::Vector(_) => ValueCategory::Vector,
            Value::String(_) => ValueCategory::String,
        }
    }

    pub fn match_category(&self, value_category: ValueCategory) -> bool {
        match self {
            Value::Scalar(_) => value_category == ValueCategory::Scalar,
            Value::Array(_) => value_category == ValueCategory::Array,
            Value::Matrix(_) => value_category == ValueCategory::Matrix,
            Value::Vector(_) => value_category == ValueCategory::Vector,
            Value::String(_) => value_category == ValueCategory::String,
        }
    }
}