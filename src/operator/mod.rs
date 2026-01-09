// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

pub mod table;
pub mod implementation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Juxtaposition,
    ArithmeticAdd,
    ArithmeticSub,
    ArithmeticMul,
    ArithmeticDiv,
    ArithmeticPow,
    VectorDot,
    VectorDet,
    VectorCross,
    BooleanNot,
    BooleanAnd,
    BooleanOr,
    BooleanXor,
    ComparatorEqual,
    ComparatorNotEqual,
    ComparatorGreaterOrEqual,
    ComparatorLessOrEqual,
    ComparatorGreaterThan,
    ComparatorLessThan,
    ComparatorIn,
    ComparatorHas,
    ComparatorIs,
    ConstructorTable,
    ConstructorList,
    ConstructorMatrix,
    ConstructorVector,
}