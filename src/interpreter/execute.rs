// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use super::value::Value;
use super::ast::ast_node::AstNode;
use super::error::collector::*;
use super::scope::*;
use super::scope::output::*;
use super::parser::*;
use super::parser::InvalidTokenError;

#[derive(Default, Clone)]
pub enum ProgramResult {
    #[default]
    Unexecuted,
    Executed(OutputCollector),
    InvalidTokens(InvalidTokenError),
}

pub fn execute_program(
    scope: &mut Scope,
    source: &str,
) -> ProgramResult {
    let program = parse_program(source);
    match program {
        Ok(program) => {
            let mut errors = ErrorCollector::new();
            let mut output = OutputCollector::new();
            let _ = program.evaluate(scope, &mut errors, &mut output);
            ProgramResult::Executed(output)
        }
        Err(e) => {
            ProgramResult::InvalidTokens(e)
        }
    }
}