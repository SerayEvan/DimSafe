// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::ops::{Mul, Div};
use std::cmp::{PartialEq};
use std::collections::HashMap;

const NUMBER_OF_UNITS: usize = 7;

const LENGTH_DIM: usize = 0;
const MASS_DIM: usize = 1;
const TIME_DIM: usize = 2;
const CURRENT_DIM: usize = 3;
const TEMPERATURE_DIM: usize = 4;
const AMOUNT_OF_SUBSTANCE_DIM: usize = 5;
const LUMINOUS_INTENSITY_DIM: usize = 6;

#[derive(Debug, Clone)]
pub struct UnitDimension {
    dim: [f32; NUMBER_OF_UNITS],
}

impl UnitDimension {
    const fn from_index(index: usize) -> Self {
        let mut dim = [0.0; NUMBER_OF_UNITS];
        dim[index] = 1.0;
        Self { dim }
    }
}

impl Mul<&UnitDimension> for UnitDimension {
    type Output = Self;
    fn mul(self, b: &UnitDimension) -> Self::Output {
        let mut dim = [0.0; NUMBER_OF_UNITS];
        for i in 0..NUMBER_OF_UNITS {
            dim[i] = self.dim[i] + b.dim[i];
        }
        Self {
            dim,
        }
    }
}
impl Div<&UnitDimension> for UnitDimension {
    type Output = Self;
    fn div(self, b: &UnitDimension) -> Self::Output {
        let mut dim = [0.0; NUMBER_OF_UNITS];
        for i in 0..NUMBER_OF_UNITS {
            dim[i] = self.dim[i] - b.dim[i];
        }
        Self { dim }
    }
}
impl UnitDimension {
    pub fn powf(&self, b: f32) -> Self {
        let mut dim = [0.0; NUMBER_OF_UNITS];
        for i in 0..NUMBER_OF_UNITS {
            dim[i] = dim[i] * b;
        }
        Self { dim }
    }
}
impl PartialEq for UnitDimension {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..NUMBER_OF_UNITS {
            if self.dim[i] != other.dim[i] {
                return false;
            }
        }
        true
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Debug, Clone)]
pub enum Unit {
    Define {
        factor: f32,
        dimension: UnitDimension,
    },
    Unmonitor,
    Error,
}
impl Unit {
    fn from_index(index: usize) -> Self {
        Self::Define {
            factor: 1.0,
            dimension: UnitDimension::from_index(index),
        }
    }
    fn derive_from_label(dim_dict: &mut HashMap<String, Unit>, unit_dim: &Unit, label: &str) {
        let pow_prefix = ["f", "p", "n", "µ", "m", "c", "d", "", "da", "h", "k", "M", "G", "T", "P"];
        let pow_values = [1e-15, 1e-12, 1e-9, 1e-6, 1e-3, 1e-2, 1e-1, 1.0, 1e1, 1e2, 1e3, 1e6, 1e9, 1e12, 1e15];
        for i in 0..pow_prefix.len() {
            let p_lab = pow_prefix[i];
            let p_val = pow_values[i];
            let unit = unit_dim.clone() * p_val;
            dim_dict.insert(p_lab.to_string() + label, unit);
        }
    }
}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Define { factor: f1, dimension: d1 }, Self::Define { factor: f2, dimension: d2 }) => f1 == f2 && d1 == d2,
            (Self::Error, _) => true,
            (Self::Unmonitor, _) => true,
            (_, Self::Error) => true,
            (_, Self::Unmonitor) => true,
        }
    }
}
impl Mul<&Unit> for Unit {
    type Output = Self;
    fn mul(self, b: &Unit) -> Self::Output {
        match (self, b) {
            (Self::Define { factor: f1, dimension: d1 }, Self::Define { factor: f2, dimension: d2 }) => {
                Self::Define {
                    factor: f1 * f2,
                    dimension: d1 * &d2,
                }
            }
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Mul<f32> for Unit {
    type Output = Self;
    fn mul(self, b: f32) -> Self::Output {
        match self {
            Self::Define { factor: f, dimension: d } => Self::Define {
                factor: f * b,
                dimension: d,
            },
            Self::Unmonitor => Self::Unmonitor,
            Self::Error => Self::Error,
        }
    }
}
impl Div<&Unit> for Unit {
    type Output = Self;
    fn div(self, b: &Unit) -> Self::Output {
        match (self, b) {
            (Self::Define { factor: f1, dimension: d1 }, Self::Define { factor: f2, dimension: d2 }) => {
                Self::Define {
                    factor: f1 / f2,
                    dimension: d1 / &d2,
                }
            },
            (Self::Error, _) => Self::Error,
            (_, Self::Error) => Self::Error,
            (Self::Unmonitor, _) => Self::Unmonitor,
            (_ , Self::Unmonitor) => Self::Unmonitor
        }
    }
}
impl Div<f32> for Unit {
    type Output = Self;
    fn div(self, b: f32) -> Self::Output {
        match self {
                Self::Define { factor: f, dimension: d } => Self::Define {
                factor: f / b,
                dimension: d,
            },
            Self::Unmonitor => Self::Unmonitor,
            Self::Error => Self::Error,
        }
    }
}
impl Unit {
    pub fn powf(&self, b: f32) -> Self {
        match self {
            Self::Define { factor: f, dimension: d } => Self::Define {
                factor: f.powf(b),
                dimension: d.powf(b),
            },
            Self::Unmonitor => Self::Unmonitor,
            Self::Error => Self::Error,
        }
    }
}

pub const DEFAULT_UNIT: Unit = Unit::Define {
    factor: 1.0,
    dimension: UnitDimension { dim: [0.0; NUMBER_OF_UNITS] },
};

fn make_unit_dictionary() -> HashMap<String, Unit> {

    let mut dict = HashMap::new();

    let meters_dim = Unit::from_index(LENGTH_DIM);
    Unit::derive_from_label(&mut dict, &meters_dim, "m");
    let seconds_dim = Unit::from_index(TIME_DIM);
    Unit::derive_from_label(&mut dict, &seconds_dim, "s");
    let kilograms_dim = Unit::from_index(MASS_DIM);
    Unit::derive_from_label(&mut dict, &kilograms_dim, "kg");
    let amperes_dim = Unit::from_index(CURRENT_DIM);
    Unit::derive_from_label(&mut dict, &amperes_dim, "A");
    let kelvins_dim = Unit::from_index(TEMPERATURE_DIM);
    Unit::derive_from_label(&mut dict, &kelvins_dim, "K");
    let moles_dim = Unit::from_index(AMOUNT_OF_SUBSTANCE_DIM);
    Unit::derive_from_label(&mut dict, &moles_dim, "mol");
    let candelas_dim = Unit::from_index(LUMINOUS_INTENSITY_DIM);
    Unit::derive_from_label(&mut dict, &candelas_dim, "cd");

    let watts_dim = meters_dim.powf(2.0) * &kilograms_dim * &seconds_dim.powf(-3.0);
    Unit::derive_from_label(&mut dict, &watts_dim, "W");

    let joules_dim = meters_dim.powf(2.0) * &kilograms_dim * &seconds_dim.powf(-2.0);
    Unit::derive_from_label(&mut dict, &joules_dim, "J");

    let newtons_dim = seconds_dim.powf(-2.0) * &meters_dim * &kilograms_dim;
    Unit::derive_from_label(&mut dict, &newtons_dim, "N");

    let pascals_dim = meters_dim.powf(-1.0) * &kilograms_dim * &seconds_dim.powf(-2.0);
    Unit::derive_from_label(&mut dict, &pascals_dim, "Pa");

    let volts_dim = meters_dim.powf(2.0) * &kilograms_dim * &seconds_dim.powf(-3.0) * &amperes_dim.powf(-1.0);
    Unit::derive_from_label(&mut dict, &volts_dim, "V");

    let ohms_dim = amperes_dim.powf(-1.0) * &volts_dim;
    Unit::derive_from_label(&mut dict, &ohms_dim, "Ohm");

    let teslas_dim = seconds_dim.powf(-2.0) * &kilograms_dim * &amperes_dim.powf(-1.0);
    Unit::derive_from_label(&mut dict, &teslas_dim, "T");

    let hertzs_dim = seconds_dim.powf(-1.0);
    Unit::derive_from_label(&mut dict, &hertzs_dim, "Hz");

    let no_dim = Unit::from_index(0);
    dict.insert("tour".to_string(), no_dim.clone() * std::f32::consts::PI * 2.0);
    dict.insert("rad".to_string(), no_dim.clone());
    dict.insert("deg".to_string(), no_dim * std::f32::consts::PI / 180.0);

    dict.insert("minute".to_string(), seconds_dim.clone() * 60.0);
    dict.insert("hour".to_string(), seconds_dim * 3600.0);

    dict.insert("error".to_string(), Unit::Error);
    dict.insert("unmonitor".to_string(), Unit::Unmonitor);

    dict
}

/*pub fn parse_unit(input: &str) -> Option<Unit> {

    // split string in 2 parts : [unit] [exponent with optional + or -]
    let mut split_idx = 0;
    for c in input.chars() {
        if c == '+' || c == '-' || c < '0' || c > '9' {
            break;
        }
        split_idx += 1;
    }

    let unit = find_unit(&input[..split_idx])?;
    if split_idx == input.len() { return Some(unit); }

    let exponent = input[split_idx..].parse::<i32>().ok()? as f32;

    Some(unit.powf(exponent))
}*/

/*

        | n*n*row | n*n*col | n*n*block | n*n*cell
choice

16*16*16*16

*/