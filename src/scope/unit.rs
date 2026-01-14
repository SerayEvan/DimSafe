// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt;
use std::fmt::Display;
use std::ops::{Mul, Div, Add, Sub};
use std::cmp::{PartialEq};
use std::collections::HashMap;
use lazy_static::lazy_static;

use crate::error::collector::*;
use crate::error::*;

const NUMBER_OF_UNITS: usize = 7;

const LENGTH_DIM: usize = 0;
const MASS_DIM: usize = 1;
const TIME_DIM: usize = 2;
const CURRENT_DIM: usize = 3;
const TEMPERATURE_DIM: usize = 4;
const AMOUNT_OF_SUBSTANCE_DIM: usize = 5;
const LUMINOUS_INTENSITY_DIM: usize = 6;

#[derive(Debug, Clone, Copy)]
pub enum UnitDimension {
    Define ( [f32; NUMBER_OF_UNITS] ),
    Error,
    Unmonitor,
}

impl UnitDimension {
    const fn no_dim() -> Self {
        Self::Define([0.0; NUMBER_OF_UNITS])
    }
    const fn from_index(index: usize) -> Self {
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
            Self::Error => write!(f, "Error"),
            Self::Unmonitor => write!(f, "Unmonitor"),
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

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub factor: f64,
    pub dimension: UnitDimension,
}
impl Unit {
    const fn no_dim() -> Self {
        Self {
            factor: 1.0,
            dimension: UnitDimension::no_dim(),
        }
    }
    const fn from_index(index: usize) -> Self {
        Self {
            factor: 1.0,
            dimension: UnitDimension::from_index(index),
        }
    }
}
impl Mul<Unit> for Unit {
    type Output = Self;
    fn mul(self, b: Unit) -> Self::Output {
        Self {
            factor: self.factor * b.factor,
            dimension: self.dimension * b.dimension,
        }
    }
}
impl Mul<f64> for Unit {
    type Output = Self;
    fn mul(self, b: f64) -> Self::Output {
        Self {
            factor: self.factor * b,
            dimension: self.dimension,
        }
    }
}
impl Div<Unit> for Unit {
    type Output = Self;
    fn div(self, b: Unit) -> Self::Output {
        Self {
            factor: self.factor / b.factor,
            dimension: self.dimension / b.dimension,
        }
    }
}
impl Div<f64> for Unit {
    type Output = Self;
    fn div(self, b: f64) -> Self::Output {
        Self {
            factor: self.factor / b,
            dimension: self.dimension / b,
        }
    }
}
impl Div<Unit> for f64 {
    type Output = Unit;
    fn div(self, b: Unit) -> Self::Output {
        Unit {
            factor: self / b.factor,
            dimension: self / b.dimension,
        }
    }
}
impl Unit {
    pub fn powf(&self, b: f64) -> Self {
        Self {
            factor: self.factor.powf(b),
            dimension: self.dimension.powf(b),
        }
    }
}

pub const DEFAULT_UNIT: Unit = Unit {
    factor: 1.0,
    dimension: UnitDimension::Define([0.0; NUMBER_OF_UNITS]),
};

pub const ERROR_UNIT: Unit = Unit {
    factor: 1.0,
    dimension: UnitDimension::Error,
};
pub const UNMONITOR_UNIT: Unit = Unit {
    factor: 1.0,
    dimension: UnitDimension::Unmonitor,
};

pub const NO_DIMENSION: UnitDimension = UnitDimension::no_dim();
pub const NO_UNIT: Unit = Unit::no_dim();
const METERS_UNIT: Unit = Unit::from_index(LENGTH_DIM);
const SECONDS_UNIT: Unit = Unit::from_index(TIME_DIM);
const KILOGRAMS_UNIT: Unit = Unit::from_index(MASS_DIM);
const AMPERES_UNIT: Unit = Unit::from_index(CURRENT_DIM);
const KELVINS_UNIT: Unit = Unit::from_index(TEMPERATURE_DIM);
const MOLES_UNIT: Unit = Unit::from_index(AMOUNT_OF_SUBSTANCE_DIM);
const CANDLEAS_UNIT: Unit = Unit::from_index(LUMINOUS_INTENSITY_DIM);

fn derive_from_label(dim_dict: &mut HashMap<String, Unit>, unit_dim: Unit, label: &str) {
    let pow_prefix = ["f", "p", "n", "µ", "m", "c", "d", "", "da", "h", "k", "M", "G", "T", "P"];
    let pow_values = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3, 1e-2, 1e-1, 1.0, 1e1, 1e2, 1e3, 1e6, 1e9, 1e12, 1e15];
    for i in 0..pow_prefix.len() {
        let p_lab = pow_prefix[i];
        let p_val = pow_values[i];
        let unit = unit_dim * p_val;
        dim_dict.insert(p_lab.to_string() + label, unit);
    }
}

fn make_unit_dictionary() -> HashMap<String, Unit> {

    let mut dict = HashMap::new();

    // SI units
    derive_from_label(&mut dict, METERS_UNIT, "m");
    derive_from_label(&mut dict, SECONDS_UNIT, "s");
    derive_from_label(&mut dict, KILOGRAMS_UNIT, "kg");
    derive_from_label(&mut dict, AMPERES_UNIT, "A");
    derive_from_label(&mut dict, KELVINS_UNIT, "K");
    derive_from_label(&mut dict, MOLES_UNIT, "mol");
    derive_from_label(&mut dict, CANDLEAS_UNIT, "cd");

    let watts_dim = METERS_UNIT.powf(2.0) * KILOGRAMS_UNIT * SECONDS_UNIT.powf(-3.0);
    derive_from_label(&mut dict, watts_dim, "W");

    let joules_dim = METERS_UNIT.powf(2.0) * KILOGRAMS_UNIT * SECONDS_UNIT.powf(-2.0);
    derive_from_label(&mut dict, joules_dim, "J");

    let newtons_dim = SECONDS_UNIT.powf(-2.0) * METERS_UNIT * KILOGRAMS_UNIT;
    derive_from_label(&mut dict, newtons_dim, "N");

    let pascals_dim = METERS_UNIT.powf(-1.0) * KILOGRAMS_UNIT * SECONDS_UNIT.powf(-2.0);
    derive_from_label(&mut dict, pascals_dim, "Pa");

    let volts_dim = METERS_UNIT.powf(2.0) * KILOGRAMS_UNIT * SECONDS_UNIT.powf(-3.0) * AMPERES_UNIT.powf(-1.0);
    derive_from_label(&mut dict, volts_dim, "V");

    let ohms_dim = AMPERES_UNIT.powf(-1.0) * volts_dim;
    derive_from_label(&mut dict, ohms_dim, "Ohm");

    let teslas_dim = SECONDS_UNIT.powf(-2.0) * KILOGRAMS_UNIT * AMPERES_UNIT.powf(-1.0);
    derive_from_label(&mut dict, teslas_dim, "T");

    let hertzs_dim = SECONDS_UNIT.powf(-1.0);
    derive_from_label(&mut dict, hertzs_dim, "Hz");

    dict.insert("rpm".to_string(),  std::f64::consts::PI * 2.0 / SECONDS_UNIT / 60.0);
    dict.insert("rad".to_string(), NO_UNIT);
    dict.insert("deg".to_string(), NO_UNIT * std::f64::consts::PI / 180.0);

    dict.insert("minute".to_string(), SECONDS_UNIT * 60.0);
    dict.insert("hour".to_string(), SECONDS_UNIT * 3600.0);

    dict.insert("error".to_string(), ERROR_UNIT);
    dict.insert("unmonitor".to_string(), UNMONITOR_UNIT);

    dict
}

lazy_static! {
    pub static ref UNIT_DICTIONARY: HashMap<String, Unit> = make_unit_dictionary();
}