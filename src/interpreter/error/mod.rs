// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod collector;

use super::ast::location::*;
use super::scope::*;
use super::unit::dimension::*;
use std::sync::Arc;

pub trait ErrorMessage {
    fn raise<T>(self) -> Result<T, Error>;
    fn name(&self) -> &'static str;
    fn description(&self) -> String;

    fn get_message(&self, loc_range: Option<RangeIndex>) -> String {
        match loc_range {
            Some(loc_range) => format!("{}: {}", self.name(), self.description()),
            None => format!("{}: {}", self.name(), self.description()),
        }
    }
}

trait ErrorType: Send + Sync {
    fn get_message(&self, loc_range: Option<RangeIndex>) -> String;
}

impl<T: ErrorMessage + Send + Sync> ErrorType for T {
    fn get_message(&self, loc_range: Option<RangeIndex>) -> String {
        self.get_message(loc_range)
    }
}

#[derive(Clone)]
pub struct Error {
    loc_range: Option<RangeIndex>,
    error_type: Arc<dyn ErrorType + Send + Sync>,
}

impl Error {
    pub fn new<M: 'static + ErrorMessage + Send + Sync>(error_type: M) -> Self {
        Self { loc_range: None, error_type: Arc::new(error_type) }
    }
    pub fn get_message(&self) -> String {
        self.error_type.get_message(self.loc_range.clone())
    }
    pub fn set_loc_range(&mut self, loc_range: &RangeIndex) {
        if self.loc_range.is_none() {
            self.loc_range = Some(loc_range.clone());
        }
    }
}

#[derive(Clone)]
pub struct UnsupportedError {
    pub functionality: &'static str,
}

impl ErrorMessage for UnsupportedError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "UnsupportedError"
    }
    fn description(&self) -> String {
        format!("The functionality '{}' is not supported", self.functionality.to_string())
    }
}

#[derive(Clone)]
pub struct AssignmentError {
    pub variable_name: String,
    pub assignment_operator: AssignmentOperator,
}

impl ErrorMessage for AssignmentError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "AssignmentError"
    }
    fn description(&self) -> String {
        match self.assignment_operator {
            AssignmentOperator::Define => format!("You can't define a variable that already exists: '{}'", self.variable_name),
            AssignmentOperator::Reassign => format!("You can't reassign a variable that is not defined: '{}'", self.variable_name),
            AssignmentOperator::Push => format!("You can't push a value to a variable that is not defined: '{}'", self.variable_name),
        }
    }
}

#[derive(Clone)]
pub struct UndefinedError {
    pub variable_name: String,
}

impl ErrorMessage for UndefinedError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "UndefinedError"
    }
    fn description(&self) -> String {
        format!("The variable '{}' is not defined", self.variable_name)
    }
}

#[derive(Clone)]
pub struct UnfoundUnitError {
    pub unit_name: String,
}

impl ErrorMessage for UnfoundUnitError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "UnfoundUnitError"
    }
    fn description(&self) -> String {
        format!("The unit '{}' is not found", self.unit_name)
    }
}

#[derive(Clone)]
pub struct DimensionMismatchError {
    pub unit_a: UnitDimension,
    pub unit_b: UnitDimension,
}

impl ErrorMessage for DimensionMismatchError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "DimensionMismatchError"
    }
    fn description(&self) -> String {
        format!("The units '{}' and '{}' do not match", self.unit_a, self.unit_b)
    }
}

#[derive(Clone)]
pub struct InvalidInputError {
    pub message: String,
}

impl ErrorMessage for InvalidInputError {
    fn raise<T>(self) -> Result<T, Error> {
        Err(Error::new(self))
    }
    fn name(&self) -> &'static str {
        "InvalidInputError"
    }
    fn description(&self) -> String {
        format!("'{}'", self.message)
    }
}
