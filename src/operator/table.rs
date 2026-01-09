// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::collections::HashMap;

use crate::error::*;
use crate::error::collector::*;
use crate::scope::value::*;
use super::implementation::*;
use super::*;

use lazy_static::lazy_static;

pub struct OperatorTable {
    implementations: HashMap<Operator, Vec<OperatorImplementation>>,
}

impl OperatorTable {

    pub fn new() -> Self {
        Self { implementations: HashMap::new() }
    }

    pub fn add_implementation(&mut self, operator: Operator, implementation: OperatorImplementation) {
        if let Some(implementations) = self.implementations.get_mut(&operator) {
            implementations.push(implementation);
        } else {
            self.implementations.insert(operator, vec![implementation]);
        }
    }
    
    pub fn compute(&self, operator: &Operator, inputs: &[Value], errors: &mut ErrorCollector) -> Value {

        // spread a previous failure and stop computation
        for input in inputs {
            match input {
                Value::Failed => return Value::Failed,
                _ => {},
            }
        }

        // try to find a matching implementation
        if let Some(implementations) = self.implementations.get(operator) {
            for implementation in implementations.iter() {
                if implementation.matches(inputs) {
                    match (implementation.implementation)(inputs) {
                        Ok(result) => return result,
                        Err(error) => {
                            errors.add_error(error);
                            return Value::Failed;
                        }
                    }
                }
            }
        }

        // if no implementation is found, raise an error
        errors.raise(UnsupportedError{functionality: "operator for given inputs"});
        Value::Failed
    }
}

lazy_static! {
    pub static ref OPERATOR_TABLE: OperatorTable = OperatorTable::new();
}

