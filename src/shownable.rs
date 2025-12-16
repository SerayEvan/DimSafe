// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::Debug;

use crate::ast::*;
use crate::ast::location::*;
use crate::ast::ast_node::*;
use crate::scope::*;
use crate::scope::operator::*;

trait Shownable {
    fn display(&self, text: &mut String, indent: &str, return_to_line: bool);
}

macro_rules! impl_shownable_case {

    (
        $name:ident :: $variant:ident, 
        $text:ident, $indent:ident, $return_to_line:ident
    ) => {};

    (
        $name:ident :: $variant:ident { $( $field:ident ),* }, 
        $text:ident, $indent:ident, $return_to_line:ident
    ) => {
        $(
            $text.push_str("\n");
            $text.push_str($indent);
            $text.push_str(concat!(" - ", stringify!($field), ": "));
            $field.display($text, &($indent.to_string() + "    "), false);
        )*
    };

    (
        $name:ident :: $variant:ident ( $( $tuple_fields:ident ),* ), 
        $text:ident, $indent:ident, $return_to_line:ident
    ) => {
        $(
            $text.push_str("\n");
            $text.push_str($indent);
            $text.push_str(" - ");
            $tuple_fields.display($text, &($indent.to_string() + "    "), false);
        )*
    };
}

macro_rules! impl_shownable_enum {
    (
        $name:ident {
            $( $variant:ident  $( ( $( $tuple_fields:ident ),* ) )? $( { $( $struct_fields:ident ),* } )? ),*
            $(,)?
        }
    ) => {
        impl Shownable for $name {
            fn display(&self, text: &mut String, indent: &str, return_to_line: bool) {
                match self {
                    $(
                        $name::$variant $( ( $( $tuple_fields ),* ) )? $( { $( $struct_fields ),* } )? => {
                            if return_to_line {
                                text.push_str("\n");
                                text.push_str(indent);
                            }
                            text.push_str(concat!(stringify!($name), ".", stringify!($variant)));

                            impl_shownable_case!(
                                $name::$variant $( ( $( $tuple_fields ),* ) )? $( { $( $struct_fields ),* } )?, 
                                text, indent, return_to_line
                            );
                        }
                    ),*
                }
            }
        }
    };
}

macro_rules! impl_shownable_with_debug {
    (
        $name:ident
    ) => {
        impl Shownable for $name {
            fn display(&self, text: &mut String, _indent: &str, _return_to_line: bool) {
                text.push_str(&format!("{:?}", self));
            }
        }
    };
}

impl<T: Shownable> Shownable for Vec<T> {
    fn display(&self, text: &mut String, indent: &str, return_to_line: bool) {
        for (index,item) in self.iter().enumerate() {
            let str_index = index.to_string();
            if index != 0 || return_to_line || true {
                text.push_str("\n");
                text.push_str(&indent);
            }
            text.push_str(format!("[{}] ", str_index).as_str());
            item.display(text, &(indent.to_string() + "  "), false);
        }
    }
}

impl<T: Shownable> Shownable for Option<T> {
    fn display(&self, text: &mut String, ident: &str, return_to_line: bool) {
        match self {
            Some(item) => item.display(text, ident, return_to_line),
            None => text.push_str("None"),
        }
    }
}

impl<T: Shownable + Clone + AstNode> Shownable for Spanned<T> {
    fn display(&self,text: &mut String, ident: &str, return_to_line: bool) {
        self.value.display(text, ident, return_to_line);
    }
}

impl<T: Shownable + Clone + PartialEq + Debug> Shownable for Leaf<T> {
    fn display(&self, text: &mut String, ident: &str, return_to_line: bool) {
        self.value.display(text, ident, return_to_line);
    }
}

impl Shownable for Identifier {
    fn display(&self,text: &mut String, ident: &str, return_to_line: bool) {
        if return_to_line {
            text.push_str("\n");
            text.push_str(ident);
        }
        text.push_str("Identifier :");
        text.push_str(&self.text.join("."));
    }
}

impl_shownable_with_debug!(String);
impl_shownable_with_debug!(f32);
impl_shownable_with_debug!(f64);
impl_shownable_with_debug!(i32);
impl_shownable_with_debug!(i64);
impl_shownable_with_debug!(bool);

impl_shownable_enum!(AssignmentOperator {
    Define,
    Reassign,
    Push,
});

impl_shownable_enum!(Operator {
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
});

impl_shownable_enum!(LiteralValue {
    Integer ( value ),
    Float ( value ),
    String ( value ),
    Bool ( value ),
    Unit ( unit, exponent ),
    Empty,
    Unspecified,
    Unimplemented,
    Invalid,
});

impl_shownable_enum!(Expression {

    // Leaf
    Literal    ( value ),
    Identifier ( identifier ),

    // Non-leaf
    Operation  { operator, arguments },
    Call       { identifier, arguments },
    Array      { array, constructor },
    Assignment { identifier, assignment_operator, value },
});

pub fn get_html<T: Shownable>(shownable: &T) -> String {
    let mut text = String::new();
    shownable.display(&mut text, "", false);
    text
}