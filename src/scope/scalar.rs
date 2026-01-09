// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::error::*;

enum ScalarResult {
    Ok(Scalar),
    Invalid,
    Unimplemented,
}

pub trait ScalarOp {

    /*
    TODO: add boolean operations

    fn not(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn and(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn or(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn xor(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }

    TODO: add comparaison

    fn equal(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn not_equal(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn equal_with_tolerance(_a: &Self, _b: &Self, _tolerance: f64) -> ScalarResult { ScalarResult::Unimplemented }
    fn not_equal_with_tolerance(_a: &Self, _b: &Self, _tolerance: f64) -> ScalarResult { ScalarResult::Unimplemented }
    fn greater_or_equal(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn less_or_equal(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn greater_than(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn less_than(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    
    TODO: add date operation*/

    fn add(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn sub(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn mul(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn div(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn pow(_a: &Self, _b: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn norm(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn neg(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn conjugate(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn real(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
    fn imaginary(_a: &Self) -> ScalarResult { ScalarResult::Unimplemented }
}

/* ===================================
            Float
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    pub value: f64,
}
impl Into<Comp> for Real {
    fn into(self) -> Comp {
        Comp { real: self.value, imaginary: 0.0 }
    }
}
impl Into<Quat> for Real {
    fn into(self) -> Quat {
        Quat { v_0: self.value, v_i: 0.0, v_j: 0.0, v_k: 0.0 }
    }
}

impl ScalarOp for Real {
    fn add(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value + b.value }))
    }
    fn sub(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value - b.value }))
    }
    fn mul(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value * b.value }))
    }
    fn div(a: &Self, b: &Self) -> ScalarResult {
        let result = a.value / b.value;
        if result.is_nan() { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Real(Real { value: result }))
    }
    fn pow(a: &Self, b: &Self) -> ScalarResult {
        let result = a.value.powf(b.value);
        if result.is_nan() || result.is_infinite() { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Real(Real { value: result }))
    }
    fn norm(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value.abs() }))
    }
    fn neg(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: -a.value }))
    }
    fn conjugate(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value }))
    }
    fn real(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.value }))
    }
    fn imaginary(_a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: 0.0 }))
    }
}

/* ===================================
            Comp
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Comp {
    pub real: f64,
    pub imaginary: f64,
}
impl Into<Quat> for Comp {
    fn into(self) -> Quat {
        Quat { v_0: self.real, v_i: self.imaginary, v_j: 0.0, v_k: 0.0 }
    }
}

impl ScalarOp for Comp {
    fn add(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Comp(Comp { real: a.real + b.real, imaginary: a.imaginary + b.imaginary }))
    }
    fn sub(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Comp(Comp { real: a.real - b.real, imaginary: a.imaginary - b.imaginary }))
    }
    fn mul(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Comp(Comp { real: a.real * b.real - a.imaginary * b.imaginary, imaginary: a.real * b.imaginary + a.imaginary * b.real }))
    }
    fn div(a: &Self, b: &Self) -> ScalarResult {
        let real = a.real * b.real + a.imaginary * b.imaginary;
        let imaginary = a.imaginary * b.real - a.real * b.imaginary;
        let denominator = b.real * b.real + b.imaginary * b.imaginary;
        if denominator == 0.0 { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Comp(Comp { real: real / denominator, imaginary: imaginary / denominator }))
    }
    // TODO: support of POWER
    fn norm(a: &Self) -> ScalarResult {
        let result = f64::sqrt(a.real*a.real + a.imaginary*a.imaginary);
        if result.is_nan() || result.is_infinite() { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Real(Real { value: result }))
    }
    fn neg(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Comp(Comp { real: -a.real, imaginary: -a.imaginary }))
    }
    fn conjugate(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Comp(Comp { real: a.real, imaginary: -a.imaginary }))
    }
    fn real(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.real }))
    }
    fn imaginary(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.imaginary }))
    }
}
/* ===================================
            Quat
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Quat {
    pub v_0: f64,
    pub v_i: f64,
    pub v_j: f64,
    pub v_k: f64,
}
impl ScalarOp for Quat {
    fn add(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: a.v_0 + b.v_0, v_i: a.v_i + b.v_i, v_j: a.v_j + b.v_j, v_k: a.v_k + b.v_k }))
    }
    fn sub(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: a.v_0 - b.v_0, v_i: a.v_i - b.v_i, v_j: a.v_j - b.v_j, v_k: a.v_k - b.v_k }))
    }
    fn mul(a: &Self, b: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: a.v_0 * b.v_0 - a.v_i * b.v_i - a.v_j * b.v_j - a.v_k * b.v_k, v_i: a.v_0 * b.v_i + a.v_i * b.v_0 + a.v_j * b.v_k - a.v_k * b.v_j, v_j: a.v_0 * b.v_j - a.v_i * b.v_k + a.v_j * b.v_0 + a.v_k * b.v_i, v_k: a.v_0 * b.v_k + a.v_i * b.v_j - a.v_j * b.v_i + a.v_k * b.v_0 }))
    }
    fn div(a: &Self, b: &Self) -> ScalarResult {
        let v_0 = a.v_0 * b.v_0 + a.v_i * b.v_i + a.v_j * b.v_j + a.v_k * b.v_k;
        let v_i = a.v_0 * b.v_i - a.v_i * b.v_0 - a.v_j * b.v_k + a.v_k * b.v_j;
        let v_j = a.v_0 * b.v_j + a.v_i * b.v_k - a.v_j * b.v_0 - a.v_k * b.v_i;
        let v_k = a.v_0 * b.v_k - a.v_i * b.v_j + a.v_j * b.v_i - a.v_k * b.v_0;
        let denominator = b.v_0 * b.v_0 + b.v_i * b.v_i + b.v_j * b.v_j + b.v_k * b.v_k;
        if denominator == 0.0 { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: v_0 / denominator, v_i: v_i / denominator, v_j: v_j / denominator, v_k: v_k / denominator }))
    }
    // TODO: support of POWER
    fn norm(a: &Self) -> ScalarResult {
        let result = f64::sqrt(a.v_0*a.v_0 + a.v_i*a.v_i + a.v_j*a.v_j + a.v_k*a.v_k);
        if result.is_nan() || result.is_infinite() { return ScalarResult::Invalid; }
        ScalarResult::Ok(Scalar::Real(Real { value: result }))
    }
    fn neg(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: -a.v_0, v_i: -a.v_i, v_j: -a.v_j, v_k: -a.v_k }))
    }
    fn conjugate(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: a.v_0, v_i: -a.v_i, v_j: -a.v_j, v_k: -a.v_k }))
    }
    fn real(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Real(Real { value: a.v_0 }))
    }
    fn imaginary(a: &Self) -> ScalarResult {
        ScalarResult::Ok(Scalar::Quat(Quat { v_0: 0.0, v_i: a.v_i, v_j: a.v_j, v_k: a.v_k }))
    }
}

/* ===================================
            Scalar
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    Real(Real),
    Comp(Comp),
    Quat(Quat),
}

impl Scalar {
    fn promote(a: Self, b: Self) -> (Self, Self) {
        match (a, b) {

            (Scalar::Real(a), Scalar::Real(b)) => (Scalar::Real(a), Scalar::Real(b)),
            (Scalar::Real(a), Scalar::Comp(b)) => (Scalar::Comp(a.into()), Scalar::Comp(b)),
            (Scalar::Real(a), Scalar::Quat(b)) => (Scalar::Quat(a.into()), Scalar::Quat(b)),

            (Scalar::Comp(a), Scalar::Real(b)) => (Scalar::Comp(a), Scalar::Comp(b.into())),
            (Scalar::Comp(a), Scalar::Comp(b)) => (Scalar::Comp(a), Scalar::Comp(b)),
            (Scalar::Comp(a), Scalar::Quat(b)) => (Scalar::Quat(a.into()), Scalar::Quat(b)),
            
            (Scalar::Quat(a), Scalar::Real(b)) => (Scalar::Quat(a), Scalar::Quat(b.into())),
            (Scalar::Quat(a), Scalar::Comp(b)) => (Scalar::Quat(a), Scalar::Quat(b.into())),
            (Scalar::Quat(a), Scalar::Quat(b)) => (Scalar::Quat(a), Scalar::Quat(b)),
        }
    }


}