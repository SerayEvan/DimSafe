// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::ast::Operator;

use super::value::Value;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FuncName {
    Operator(Operator),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FuncFootprint {
    name: FuncName,
    arguments_type: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function {
    name: FuncName,
    arguments_type: Vec<usize>,
    body: Expression,
}