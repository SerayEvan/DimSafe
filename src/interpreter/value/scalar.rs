// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt;
use std::fmt::Display;

use super::super::operator::table::*;
use super::super::error::*;
use super::super::operator::implementation::*;
use super::super::operator::*;
use super::super::unit::dimension::*;

use super::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub value: f64,
    pub unit: UnitDimension,
}

impl Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.unit == NO_DIMENSION {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.unit)
        }
    }
}

impl ValueTrait for Scalar {
    fn try_from_value(value: &Value) -> Option<&Self> {
        match value {
            Value::Scalar(scalar) => Some(scalar),
            _ => None
        }
    }
}

pub fn scalar_operator(op_table: &mut OperatorTable) {
    op_table.add_implementation(
        Operator::ArithmeticAdd, 
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            true,
            |inputs, errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                UnitDimension::verify(&a.unit, &b.unit, errors);
                Value::Scalar(Scalar{value: a.value + b.value, unit: a.unit + b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::ArithmeticSub, 
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            true,
            |inputs, errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                UnitDimension::verify(&a.unit, &b.unit, errors);
                Value::Scalar(Scalar{value: a.value - b.value, unit: a.unit - b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::ArithmeticMul, 
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            true,
            |inputs, _errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                Value::Scalar(Scalar{value: a.value * b.value, unit: a.unit * b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::Juxtaposition, 
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            true,
            |inputs, _errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                Value::Scalar(Scalar{value: a.value * b.value, unit: a.unit * b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::ArithmeticDiv, 
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            false,
            |inputs, errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                if b.value == 0.0 {
                    errors.raise(InvalidInputError{message: "Division by zero".to_string()});
                    return Value::Failed;
                }
                Value::Scalar(Scalar{value: a.value / b.value, unit: a.unit / b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::Shown,
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            false,
            |inputs, errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                if b.value == 0.0 {
                    errors.raise(InvalidInputError{message: "Division by zero".to_string()});
                    return Value::Failed;
                }
                Value::Scalar(Scalar{value: a.value / b.value, unit: a.unit / b.unit})
            }
        )
    );
    op_table.add_implementation(
        Operator::ArithmeticPow,
        OperatorImplementation::new(
            vec![TypeId::Scalar, TypeId::Scalar],
            false,
            |inputs, errors| {
                let a = Scalar::try_from_value(&inputs[0]).unwrap();
                let b = Scalar::try_from_value(&inputs[1]).unwrap();
                let result = a.value.powf(b.value);
                if !result.is_finite() {
                    errors.raise(InvalidInputError{message: "Power operation".to_string()});
                    return Value::Failed;
                }
                Value::Scalar(Scalar{value: result, unit: a.unit.powf(b.value)})
            }
        )
    );

}