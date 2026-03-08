// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use super::super::ast::location::*;

#[derive(Clone)]
pub enum OutputMessage {
    Result(String),
    Error(String),
}

#[derive(Clone)]
pub struct Output {
    pub initial_location: LocationIndex, // location in text when the interpreter interpreted the text
    pub index: usize, // index used to place the ghost overlay even if the text is modified
    pub message: OutputMessage,
}

impl Output {
    pub fn new(location: LocationIndex, message: OutputMessage, index: usize) -> Self {
        Self { initial_location: location, index, message }
    }
}

#[derive(Clone)]
pub struct OutputCollector {
    pub outputs: Vec<Output>,
    nb_ghost_overlay: usize,
}

impl OutputCollector {
    pub fn new() -> Self {
        Self { outputs: Vec::new(), nb_ghost_overlay: 0 }
    }

    pub fn add(&mut self, location: LocationIndex, message: OutputMessage) {
        self.outputs.push(Output::new(location, message, self.nb_ghost_overlay));
        self.nb_ghost_overlay += 1;
    }
}
