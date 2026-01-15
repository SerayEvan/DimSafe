// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt;
use std::fmt::Display;
use std::ops::{Mul, Div, Add, Sub};
use std::cmp::{PartialEq};

use super::super::error::*;
use super::super::error::collector::*;

pub const NUMBER_OF_UNITS: usize = 7;

pub const LENGTH_DIM: usize = 0;
pub const MASS_DIM: usize = 1;
pub const TIME_DIM: usize = 2;
pub const CURRENT_DIM: usize = 3;
pub const TEMPERATURE_DIM: usize = 4;
pub const AMOUNT_OF_SUBSTANCE_DIM: usize = 5;
pub const LUMINOUS_INTENSITY_DIM: usize = 6;

#[derive(Debug, Clone, Copy)]
pub enum UnitDimension {
    Define ( [f32; NUMBER_OF_UNITS] ),
    Error,
    Unmonitor,
}

impl UnitDimension {
    pub const fn no_dim() -> Self {
        Self::Define([0.0; NUMBER_OF_UNITS])
    }
    pub const fn from_index(index: usize) -> Self {
        let mut dim = [0.0; NUMBER_OF_UNITS];
        dim[index] = 1.0;
        Self::Define(dim)
    }
    pub fn verify(a: &Self, b: &Self, errors: &mut ErrorCollector) -> bool {
        match (a, b) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                for i in 0..NUMBER_OF_UNITS {
                    if dim1[i] != dim2[i] {
                        errors.raise(DimensionMismatchError{unit_a: *a, unit_b: *b});
                        return false;
                    }
                }
                true
            }
            _ => true,
        }
    }
}
impl Display for UnitDimension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Define(dim) => {
                const DIMENSION_NAMES: [&str; NUMBER_OF_UNITS] = ["L", "M", "T", "I", "Θ", "N", "J"];
                write!(f, "[[")?;
                for i in 0..NUMBER_OF_UNITS {
                    if dim[i] != 0.0 {
                        write!(f, " {}{}", DIMENSION_NAMES[i], dim[i])?;
                    }
                }
                write!(f, "]]")?;
                Ok(())
            }
            Self::Error => write!(f, "[[Error]]"),
            Self::Unmonitor => write!(f, ""),
        }
    }
}
impl Add<UnitDimension> for UnitDimension {
    type Output = Self;
    fn add(self, b: UnitDimension) -> Self::Output {
        match (self, b) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                for i in 0..NUMBER_OF_UNITS {
                    if dim1[i] != dim2[i] {
                        return Self::Error;
                    }
                }
                self.clone()
            }
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Sub<UnitDimension> for UnitDimension {
    type Output = Self;
    fn sub(self, b: UnitDimension) -> Self::Output {
        match (self, b) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                for i in 0..NUMBER_OF_UNITS {
                    if dim1[i] != dim2[i] {
                        return Self::Error;
                    }
                }
                self.clone()
            }
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Mul<UnitDimension> for UnitDimension {
    type Output = Self;
    fn mul(self, b: UnitDimension) -> Self::Output {
        match (self, b) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                let mut dim = [0.0; NUMBER_OF_UNITS];
                for i in 0..NUMBER_OF_UNITS {
                    dim[i] = dim1[i] + dim2[i];
                }
                Self::Define(dim)
            }
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Div<UnitDimension> for UnitDimension {
    type Output = Self;
    fn div(self, b: UnitDimension) -> Self::Output {
        match (self, b) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                let mut dim = [0.0; NUMBER_OF_UNITS];
                for i in 0..NUMBER_OF_UNITS {
                    dim[i] = dim1[i] - dim2[i];
                }
                Self::Define(dim)
            }
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Div<f64> for UnitDimension {
    type Output = Self;
    fn div(self, _: f64) -> Self::Output {
        self
    }
}
impl Div<UnitDimension> for f64 {
    type Output = UnitDimension;
    fn div(self, b: UnitDimension) -> Self::Output {
        match b {
            UnitDimension::Define(dim) => {
                let mut new_dim = [0.0; NUMBER_OF_UNITS];
                for i in 0..NUMBER_OF_UNITS {
                    new_dim[i] = 1.0 - dim[i];
                }
                UnitDimension::Define(new_dim)
            }
            UnitDimension::Error => UnitDimension::Error,
            UnitDimension::Unmonitor => UnitDimension::Unmonitor,
        }
    }
}
impl UnitDimension {
    pub fn powf(&self, b: f64) -> Self {
        match self {
            Self::Define(dim) => {
                let mut mul_dim = [0.0; NUMBER_OF_UNITS];
                for i in 0..NUMBER_OF_UNITS {
                    mul_dim[i] = dim[i] * b as f32;
                }
                Self::Define(mul_dim)
            }
            _ => self.clone(),
        }
    }
}
impl PartialEq for UnitDimension {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Define(dim1), Self::Define(dim2)) => {
                for i in 0..NUMBER_OF_UNITS {
                    if dim1[i] != dim2[i] {
                        return false;
                    }
                }
                true
            }
            (Self::Error, _) => false,
            (_, Self::Error) => false,
            (Self::Unmonitor, _) => true,
            (_ , Self::Unmonitor) => true,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

pub const NO_DIMENSION: UnitDimension = UnitDimension::no_dim();