// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::error::*;

pub trait ScalarOp {

    fn not(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "not"}.raise() }
    fn and(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "and"}.raise() }
    fn or(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "or"}.raise() }
    fn xor(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "xor"}.raise() }

    fn equal(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "equal"}.raise() }
    fn not_equal(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "not_equal"}.raise() }
    fn equal_with_tolerance(_a: &Self, _b: &Self, _tolerance: f64) -> Result<Scalar, Error> { UnsupportedError{functionality: "equal_with_tolerance"}.raise() }
    fn not_equal_with_tolerance(_a: &Self, _b: &Self, _tolerance: f64) -> Result<Scalar, Error> { UnsupportedError{functionality: "not_equal_with_tolerance"}.raise() }
    fn greater_or_equal(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "greater_or_equal"}.raise() }
    fn less_or_equal(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "less_or_equal"}.raise() }
    fn greater_than(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "greater_than"}.raise() }
    fn less_than(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "less_than"}.raise() }

    fn add(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "add"}.raise() }
    fn sub(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "sub"}.raise() }
    fn mul(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "mul"}.raise() }
    fn div(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "div"}.raise() }   
    fn int_div(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "int_div"}.raise() }
    fn mod_div(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "mod_div"}.raise() }
    fn pow(_a: &Self, _b: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "pow"}.raise() }
    fn norm(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "norm"}.raise() }
    fn neg(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "neg"}.raise() }
    fn conjugate(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "conjugate"}.raise() }
    fn real(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "real"}.raise() }
    fn imaginary(_a: &Self) -> Result<Scalar, Error> { UnsupportedError{functionality: "imaginary"}.raise() }
}

/* ===================================
            Boolean
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
}

impl ScalarOp for Boolean {
    fn not(a: &Self) -> Result<Scalar, Error> { Ok(Scalar::Boolean(Boolean { value: !a.value })) }
    fn and(a: &Self, b: &Self) -> Result<Scalar, Error> { Ok(Scalar::Boolean(Boolean { value: a.value && b.value })) }
    fn or(a: &Self, b: &Self) -> Result<Scalar, Error> { Ok(Scalar::Boolean(Boolean { value: a.value || b.value })) }
    fn xor(a: &Self, b: &Self) -> Result<Scalar, Error> { Ok(Scalar::Boolean(Boolean { value: a.value ^ b.value })) }
}

/* ===================================
            Integer
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
}
impl Into<Float> for Integer {
    fn into(self) -> Float {
        Float { value: self.value as f64 }
    }
}
impl Into<Complex> for Integer {
    fn into(self) -> Complex {
        Complex { real: self.value as f64, imaginary: 0.0 }
    }
}
impl Into<Quaternion> for Integer {
    fn into(self) -> Quaternion {
        Quaternion { v_0: self.value as f64, v_i: 0.0, v_j: 0.0, v_k: 0.0 }
    }
}

impl ScalarOp for Integer {
    fn add(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value + b.value }))
    }
    fn sub(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value - b.value }))
    }
    fn mul(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value * b.value }))
    }
    fn div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: (a.value as f64) / (b.value as f64) }))
    }
    fn pow(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: (a.value as f64).powf(b.value as f64) }))
    }
    fn int_div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value / b.value }))
    }
    fn mod_div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value % b.value }))
    }
    fn norm(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value.abs() }))
    }
    fn neg(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: -a.value }))
    }
    fn conjugate(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value }))
    }
    fn real(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: a.value }))
    }
    fn imaginary(_a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Integer(Integer { value: 0 }))
    }
}

/* ===================================
            Float
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub value: f64,
}
impl Into<Complex> for Float {
    fn into(self) -> Complex {
        Complex { real: self.value, imaginary: 0.0 }
    }
}
impl Into<Quaternion> for Float {
    fn into(self) -> Quaternion {
        Quaternion { v_0: self.value, v_i: 0.0, v_j: 0.0, v_k: 0.0 }
    }
}

impl ScalarOp for Float {
    fn add(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value + b.value }))
    }
    fn sub(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value - b.value }))
    }
    fn mul(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value * b.value }))
    }
    fn div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let result = a.value / b.value;
        if result.is_nan() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Float(Float { value: result }))
    }
    fn int_div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let result = f64::floor(a.value / b.value);
        if result.is_nan() || result.is_infinite() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Integer(Integer { value: result as i64 }))
    }
    fn mod_div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let result = a.value % b.value;
        if result.is_nan() || result.is_infinite() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Float(Float { value: result }))
    }
    fn pow(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let result = a.value.powf(b.value);
        if result.is_nan() || result.is_infinite() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Float(Float { value: result }))
    }
    fn norm(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value.abs() }))
    }
    fn neg(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: -a.value }))
    }
    fn conjugate(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value }))
    }
    fn real(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.value }))
    }
    fn imaginary(_a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: 0.0 }))
    }
}

/* ===================================
            Complex
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}
impl Into<Quaternion> for Complex {
    fn into(self) -> Quaternion {
        Quaternion { v_0: self.real, v_i: self.imaginary, v_j: 0.0, v_k: 0.0 }
    }
}

impl ScalarOp for Complex {
    fn add(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Complex(Complex { real: a.real + b.real, imaginary: a.imaginary + b.imaginary }))
    }
    fn sub(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Complex(Complex { real: a.real - b.real, imaginary: a.imaginary - b.imaginary }))
    }
    fn mul(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Complex(Complex { real: a.real * b.real - a.imaginary * b.imaginary, imaginary: a.real * b.imaginary + a.imaginary * b.real }))
    }
    fn div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let real = a.real * b.real + a.imaginary * b.imaginary;
        let imaginary = a.imaginary * b.real - a.real * b.imaginary;
        let denominator = b.real * b.real + b.imaginary * b.imaginary;
        if denominator == 0.0 { return Ok(Scalar::Invalid); }
        Ok(Scalar::Complex(Complex { real: real / denominator, imaginary: imaginary / denominator }))
    }
    // TODO: support of POWER
    fn norm(a: &Self) -> Result<Scalar, Error> {
        let result = f64::sqrt(a.real*a.real + a.imaginary*a.imaginary);
        if result.is_nan() || result.is_infinite() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Float(Float { value: result }))
    }
    fn neg(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Complex(Complex { real: -a.real, imaginary: -a.imaginary }))
    }
    fn conjugate(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Complex(Complex { real: a.real, imaginary: -a.imaginary }))
    }
    fn real(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.real }))
    }
    fn imaginary(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.imaginary }))
    }
}
/* ===================================
            Quaternion
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub struct Quaternion {
    pub v_0: f64,
    pub v_i: f64,
    pub v_j: f64,
    pub v_k: f64,
}
impl ScalarOp for Quaternion {
    fn add(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: a.v_0 + b.v_0, v_i: a.v_i + b.v_i, v_j: a.v_j + b.v_j, v_k: a.v_k + b.v_k }))
    }
    fn sub(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: a.v_0 - b.v_0, v_i: a.v_i - b.v_i, v_j: a.v_j - b.v_j, v_k: a.v_k - b.v_k }))
    }
    fn mul(a: &Self, b: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: a.v_0 * b.v_0 - a.v_i * b.v_i - a.v_j * b.v_j - a.v_k * b.v_k, v_i: a.v_0 * b.v_i + a.v_i * b.v_0 + a.v_j * b.v_k - a.v_k * b.v_j, v_j: a.v_0 * b.v_j - a.v_i * b.v_k + a.v_j * b.v_0 + a.v_k * b.v_i, v_k: a.v_0 * b.v_k + a.v_i * b.v_j - a.v_j * b.v_i + a.v_k * b.v_0 }))
    }
    fn div(a: &Self, b: &Self) -> Result<Scalar, Error> {
        let v_0 = a.v_0 * b.v_0 + a.v_i * b.v_i + a.v_j * b.v_j + a.v_k * b.v_k;
        let v_i = a.v_0 * b.v_i - a.v_i * b.v_0 - a.v_j * b.v_k + a.v_k * b.v_j;
        let v_j = a.v_0 * b.v_j + a.v_i * b.v_k - a.v_j * b.v_0 - a.v_k * b.v_i;
        let v_k = a.v_0 * b.v_k - a.v_i * b.v_j + a.v_j * b.v_i - a.v_k * b.v_0;
        let denominator = b.v_0 * b.v_0 + b.v_i * b.v_i + b.v_j * b.v_j + b.v_k * b.v_k;
        if denominator == 0.0 { return Ok(Scalar::Invalid); }
        Ok(Scalar::Quaternion(Quaternion { v_0: v_0 / denominator, v_i: v_i / denominator, v_j: v_j / denominator, v_k: v_k / denominator }))
    }
    // TODO: support of POWER
    fn norm(a: &Self) -> Result<Scalar, Error> {
        let result = f64::sqrt(a.v_0*a.v_0 + a.v_i*a.v_i + a.v_j*a.v_j + a.v_k*a.v_k);
        if result.is_nan() || result.is_infinite() { return Ok(Scalar::Invalid); }
        Ok(Scalar::Float(Float { value: result }))
    }
    fn neg(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: -a.v_0, v_i: -a.v_i, v_j: -a.v_j, v_k: -a.v_k }))
    }
    fn conjugate(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: a.v_0, v_i: -a.v_i, v_j: -a.v_j, v_k: -a.v_k }))
    }
    fn real(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Float(Float { value: a.v_0 }))
    }
    fn imaginary(a: &Self) -> Result<Scalar, Error> {
        Ok(Scalar::Quaternion(Quaternion { v_0: 0.0, v_i: a.v_i, v_j: a.v_j, v_k: a.v_k }))
    }
}

/* ===================================
            Scalar
=================================== */
#[derive(Debug, Clone, PartialEq)]
pub enum Scalar {
    Boolean(Boolean),
    Integer(Integer),
    Float(Float),
    Complex(Complex),
    Quaternion(Quaternion),
    Invalid,
    Unspecified,
    Unimplemented,
    Empty,
}