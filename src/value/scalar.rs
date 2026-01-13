// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use crate::operator::table::*;
use crate::error::*;
use crate::operator::implementation::*;
use crate::operator::*;
use crate::scope::unit::*;
use super::*;
use std::fmt;
use std::fmt::Display;


#[derive(Debug, Clone, PartialEq)]
pub struct Scalar {
    pub value: f64,
    pub unit: UnitDimension,
}

impl Display for Scalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.unit)
    }
}

impl ValueTrait for Scalar {
    fn type_id() -> TypeId { TypeId::Scalar }
    fn try_from_value(value: &Value) -> Option<&Self> {
        match value {
            Value::Scalar(scalar) => Some(scalar),
            _ => None
        }
    }
    fn into_value(self) -> Value {
        Value::Scalar(self)
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

}
