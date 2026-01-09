// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::Debug;

use crate::error::*;
use crate::scope::value::*;

#[derive(Debug, Clone)]
pub struct OperatorImplementation {
    inputs_type: Vec<TypeId>,
    output_type: Option<TypeId>,
    symmetric: bool,
    pub implementation: fn(&[Value]) -> Result<Value, Error>,
}

impl OperatorImplementation {

    pub fn matches(&self, inputs: &[Value]) -> bool {
        let inputs_type: Vec<TypeId> = inputs.iter().map(|input| input.get_type()).collect();
        if self.symmetric {
            return self.inputs_type.iter().eq(inputs_type.iter()) || self.inputs_type.iter().rev().eq(inputs_type.iter());
        }
        
        return self.inputs_type.iter().eq(inputs_type.iter());
    }
}