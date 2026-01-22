// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use super::super::ast::location::*;

#[derive(Clone)]
pub struct Output {
    pub initial_location: LocationIndex, // location in text when the interpreter interpreted the text
    pub index: usize, // index used to place the ghost overlay even if the text is modified
    pub text: String, 
}

impl Output {
    pub fn new(location: LocationIndex, text: String, index: usize) -> Self {
        Self { initial_location: location, index, text }
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

    pub fn add(&mut self, location: LocationIndex, text: String) {
        self.outputs.push(Output::new(location, text, self.nb_ghost_overlay));
        self.nb_ghost_overlay += 1;
    }
}
