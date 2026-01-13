use crate::ast::location::*;
use crate::editor::stylization::*;

#[derive(Clone)]
pub struct Output {
    initial_location: LocationIndex,
    ghost_overlay: GhostOverlayIndex,
    pub text: String,
}

impl Output {
    pub fn new(location: LocationIndex, text: String, ghost_overlay: GhostOverlayIndex) -> Self {
        Self { initial_location: location, ghost_overlay, text }
    }
}

#[derive(Clone)]
pub struct OutputCollector {
    outputs: Vec<Output>,
    nb_ghost_overlay: usize,
    is_placed: bool,
}

impl OutputCollector {
    pub fn new() -> Self {
        Self { outputs: Vec::new(), nb_ghost_overlay: 0, is_placed: false }
    }

    pub fn add(&mut self, location: LocationIndex, text: String) {
        let ghost_overlay = GhostOverlayIndex{index: self.nb_ghost_overlay};
        self.outputs.push(Output::new(location, text, ghost_overlay));
        self.nb_ghost_overlay += 1;
    }

    pub fn apply_on_stylization(&mut self, stylization: &mut Stylization) {
        for output in &self.outputs {
            stylization.insert_ghost_overlay(output.initial_location.index, output.ghost_overlay);
        }
        self.is_placed = true;
    }

    pub fn is_placed(&self) -> bool {
        self.is_placed
    }
}
