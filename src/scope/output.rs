use crate::ast::location::*;

#[derive(Clone)]
pub struct Output {
    pub initial_location: LocationIndex,
    pub index: usize,
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
    pub is_placed: bool,
}

impl OutputCollector {
    pub fn new() -> Self {
        Self { outputs: Vec::new(), nb_ghost_overlay: 0, is_placed: false }
    }

    pub fn add(&mut self, location: LocationIndex, text: String) {
        self.outputs.push(Output::new(location, text, self.nb_ghost_overlay));
        self.nb_ghost_overlay += 1;
    }
}
