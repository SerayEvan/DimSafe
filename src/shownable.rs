// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::fmt::Debug;

use leptos::prelude::*;

use crate::ast::*;
use crate::ast::location::*;
use crate::ast::ast_node::*;
use crate::scope::*;
use crate::scope::operator::*;

#[component]
fn TreeNode(children: Children) -> impl IntoView {
    //let is_open = RwSignal::new(false);
    view! {
        <div class="tree_node">
            /*<button onclick=move || is_open.set(!is_open.get())>
                {move || if is_open.get() { "-" } else { "+" }}
            </button>*/
            <div class="tree_node_content">
                {children()}
            </div>
        </div>
    }
}

pub trait Shownable where Self: Sized + Send + Sync + 'static {
    fn display(self) ->  impl IntoView;
}

macro_rules! impl_shownable_case {

    (
        $name:ident :: $variant:ident,
    ) => {()};

    (
        $name:ident :: $variant:ident { $( $field:ident ),* }, 
    ) => {
        view! {
            $(
                <div>
                    {stringify!($field)} ":" {$field.display()}
                </div>
            )*
        }
    };

    (
        $name:ident :: $variant:ident ( $( $field:ident ),* ), 
    ) => {
        view! {
            $(
                <div>
                    {$field.display()}
                </div>
            )*
        }
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
            fn display(self) -> impl IntoView {
                view! {
                    <TreeNode>
                        {match self {
                            $(
                                $name::$variant $( ( $( $tuple_fields ),* ) )? $( { $( $struct_fields ),* } )? => {
                                    view! {
                                        {stringify!($name)}"."{stringify!($variant)}
                                        {impl_shownable_case!(
                                            $name::$variant $( ( $( $tuple_fields ),* ) )? $( { $( $struct_fields ),* } )?, 
                                        )}
                                    }.into_any()
                                }
                            ),*
                        }}
                    </TreeNode>
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
            fn display(self) -> impl IntoView {
                view! {
                    {format!("{:?}", self)}
                }
            }
        }
    };
}

impl<T: Shownable, U: Shownable> Shownable for Result<T, U> {
    fn display(self) -> impl IntoView {
        view! {
            {match self {
                Ok(item) => item.display().into_any(),
                Err(item) => item.display().into_any(),
            }}
        }
    }
}

impl<T: Shownable> Shownable for Vec<T> {
    fn display(self) -> impl IntoView {
        view! {
            <TreeNode>
                {
                    self.into_iter().enumerate().map(|(index,item)| {
                        let str_index = index.to_string();
                        view! {
                            <div>
                                {str_index}":" {item.display()}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }
            </TreeNode>
        }
    }
}

impl<T: Shownable + Clone> Shownable for Option<T> {
    fn display(self) -> impl IntoView {
        view! {
            {match self {
                Some(item) => item.display().into_any(),
                None => view! { "None" }.into_any(),
            }}
        }
    }
}

impl<T: Shownable + Clone + AstNode> Shownable for Spanned<T> {
    fn display(self) -> impl IntoView {
        view! {
            {self.value.display()}
        }
    }
}

impl<T: Shownable + Clone + PartialEq + Debug> Shownable for Leaf<T> {
    fn display(self) -> impl IntoView {
        view! {
            {self.value.display()}
        }
    }
}

impl Shownable for Identifier {
    fn display(self) -> impl IntoView {
        view! {
            "Identifier : " {self.text.join(".")}
        }
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
    Identifier ( ident ),

    // Non-leaf
    Operation  { op, args },
    Call       { ident, args },
    Array      { arr, op },
    Assignment { op, ident, value },
});