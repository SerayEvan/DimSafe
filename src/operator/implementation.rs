// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::Debug;

use crate::error::collector::*;
use crate::value::*;

#[derive(Debug, Clone)]
pub struct OperatorImplementation {
    inputs_type: Vec<TypeId>,
    symmetric: bool,
    pub implementation: fn(&[Value], &mut ErrorCollector) -> Value,
}

impl OperatorImplementation {

    pub fn new(inputs_type: Vec<TypeId>, symmetric: bool, implementation: fn(&[Value], &mut ErrorCollector) -> Value) -> Self {
        Self { inputs_type, symmetric, implementation }
    }

    pub fn matches(&self, inputs: &[Value]) -> bool {
        let inputs_type: Vec<TypeId> = inputs.iter().map(|input| input.get_type()).collect();
        if self.symmetric {
            return self.inputs_type.iter().eq(inputs_type.iter()) || self.inputs_type.iter().rev().eq(inputs_type.iter());
        }
        
        return self.inputs_type.iter().eq(inputs_type.iter());
    }
}