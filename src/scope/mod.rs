// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod scalar;
pub mod value;
//pub mod function;
pub mod unit;
pub mod operator;

use std::collections::HashMap;

use crate::error::*;

use super::value::*;
use super::unit::*;

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
    units: HashMap<String, Unit>,
    //functions: HashMap<String, Function>,
    namespaces: HashMap<String, Scope>,
}

impl Scope {
    pub fn new() -> Self {
        Self { variables: HashMap::new(), units: HashMap::new(), namespaces: HashMap::new() }
    }
    pub fn merge(&self, other: &Self) -> Self {
        let mut variables = self.variables.clone();
        variables.extend(other.variables.clone());
        let mut units = self.units.clone();
        units.extend(other.units.clone());
        let mut namespaces = self.namespaces.clone();
        namespaces.extend(other.namespaces.clone());
        Self { variables, units, namespaces }
    }

    pub fn get_namespace(&self, key: &Identifier) -> Result<&Scope, Error> {
        if key.text.len() == 1 {
            return Ok(self);
        }
        match self.namespaces.get(&key.text[0]) {
            Some(namespace) => Ok(namespace),
            None => UndifineError{variable_name: key.text[0].clone()}.raise(),
        }
    }

    pub fn get_value(&self, key: &Identifier) -> Result<Value, Error> {
        let namespace = self.get_namespace(key)?;
        match namespace.variables.get(&key.text.last().unwrap().clone()) {
            Some(value) => Ok(value.clone()),
            None => UndifineError{variable_name: key.text.last().unwrap().clone()}.raise(),
        }
    }
    
    pub fn get_unit(&self, key: &Identifier) -> Result<Unit, Error> {
        let namespace = self.get_namespace(key)?;
        match namespace.units.get(&key.text.last().unwrap().clone()) {
            Some(unit) => Ok(unit.clone()),
            None => UndifineError{variable_name: key.text.last().unwrap().clone()}.raise(),
        }
    }

    /*pub fn get_function(&self, key: &Identifier) -> Result<Function, Error> {
        let namespace = self.get_namespace(key)?;
        match namespace.functions.get(&key.text.last().unwrap().clone()) {
            Some(function) => Ok(function.clone()),
            None => UndifineError{variable_name: key.text.last().unwrap().clone()}.raise(),
        }*/

    pub fn assign_value(&mut self, key: &Identifier, value: Value, assignment_operator: &AssignmentOperator) -> Result<(), Error> {
        // reassigning a value of a variable only if it exists to override it
        let namespace = self.get_namespace(key)?;
        let last_key = key.text.last().unwrap().clone();

        let override_value = match assignment_operator {
            AssignmentOperator::Define => true,
            AssignmentOperator::Reassign => true,
            AssignmentOperator::Push => false,
        };
        if override_value == namespace.variables.contains_key(&last_key) {
            return AssignmentError{variable_name: last_key, assignment_operator: assignment_operator.clone()}.raise();
        }
        self.variables.insert(last_key, value);
        Ok(())
    }
}