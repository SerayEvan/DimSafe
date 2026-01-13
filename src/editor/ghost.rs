use wasm_bindgen::JsCast;
use web_sys::{Element, Node};

use crate::editor::stylization::*;
use log::info;

const GHOST_BEGIN_MARKER: &str = "\u{e003}";
const GHOST_END_MARKER: &str = "\u{e004}";

#[derive(Debug)]
pub struct GhostReversePlacement {
    marker: Vec<(usize, GhostOverlayIndex)>,
}

impl GhostReversePlacement {

    pub fn insert_marker(node: &Node) {
        // insert GHOST_BEGIN_MARKER and GHOST_END_MARKER for all span with class ghost_overlay in children of node_ref
        let element = node.dyn_ref::<Element>().expect("Cannot convert to Element");
        let spans = element.get_elements_by_class_name("ghost_overlay");
        for i in 0..spans.length() {
            let span = spans.item(i).expect("Cannot get span");
            let span = span.dyn_ref::<Element>().expect("Cannot convert to Element");

            // get index of id
            let id = span.id();
            let index = id.split("ghost_overlay_").last().expect("Cannot get index").parse::<usize>().expect("Cannot parse index");

            let text = GHOST_BEGIN_MARKER.to_string() + &index.to_string() + GHOST_END_MARKER;
            span.set_inner_html(&text);
        }
    }

    pub fn retrieve_ghost_overlay(text: &str) -> (Self, String) {
        let mut ghost_reverse_placement = GhostReversePlacement{marker: vec![]};
        let mut new_string = String::new();
        let mut last_cut_index = 0;
        loop {

            // find start index of ghost begin marker
            let start_index = text[last_cut_index..].find(GHOST_BEGIN_MARKER);
            if start_index.is_none() { break; }
            let start_index = start_index.unwrap() + last_cut_index;

            // find end index of ghost end marker
            let end_index = text[start_index..].find(GHOST_END_MARKER);
            if end_index.is_none() { break; }
            let end_index = end_index.unwrap() + start_index;

            // get index of ghost overlay
            let str_index = &text[start_index+GHOST_BEGIN_MARKER.len()..end_index];
            let index = str_index.parse::<usize>().expect("Cannot parse index");

            // add text to new string
            new_string += &text[last_cut_index..start_index];
            
            ghost_reverse_placement.marker.push((new_string.len(), GhostOverlayIndex{index: index}));

            last_cut_index = end_index + GHOST_END_MARKER.len();
        }
        // add remaining text to new string
        new_string += &text[last_cut_index..];

        info!("ghost_reverse_placement: {:?}", ghost_reverse_placement);

        (ghost_reverse_placement, new_string)
    }

    pub fn restore_ghost_overlay(&self, stylization: &mut Stylization) {
        info!("restore_ghost_overlay: {:?}", self);
        for (position, ghost_overlay) in &self.marker {
            stylization.insert_ghost_overlay(*position, *ghost_overlay);
        }
    }
}