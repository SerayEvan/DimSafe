// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod scalar;
pub mod value;
//pub mod function;
pub mod unit;

use std::collections::HashMap;

use crate::error::*;
use crate::error::collector::*;
use super::value::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AssignmentOperator {
    Define,
    Reassign,
    Push,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub text: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    variables: HashMap<String, Value>,
}

impl Scope {
    pub fn new() -> Self {
        Self { variables: HashMap::new() }
    }
    pub fn merge(&self, other: &Self) -> Self {
        let mut variables = self.variables.clone();
        variables.extend(other.variables.clone());
        Self { variables }
    }

    pub fn get_value(&self, key: &Identifier, errors: &mut ErrorCollector) -> Value {
        match self.variables.get(&key.text.last().unwrap().clone()) {
            Some(value) => value.clone(),
            None => {
                errors.raise(UndefinedError{variable_name: key.text.last().unwrap().clone()});
                Value::Failed
            }
        }
    }

    pub fn assign_value(&mut self, key: &Identifier, value: Value, assignment_operator: &AssignmentOperator, errors: &mut ErrorCollector) {
        // reassigning a value of a variable only if it exists to override it
        let last_key = key.text.last().unwrap().clone();

        let override_value = match assignment_operator {
            AssignmentOperator::Define => true,
            AssignmentOperator::Reassign => true,
            AssignmentOperator::Push => false,
        };
        if override_value == self.variables.contains_key(&last_key) {
            errors.raise(AssignmentError{variable_name: last_key, assignment_operator: assignment_operator.clone()});
        }
        else {
            self.variables.insert(last_key, value);
        }
    }
}