// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use wasm_bindgen::JsCast;
use web_sys::{Element, Node};
use leptos::*;

use crate::interpreter::scope::output::*;

use super::marking::*;
use super::cursor::{CURSOR_BEGIN_MARKER, CURSOR_END_MARKER};

const OVERLAYS_BEGIN_MARKER: char = '\u{e003}';
const OVERLAYS_END_MARKER: char = '\u{e004}';

#[derive(Debug)]
pub struct OverlaysLocation {
    location: Vec<(usize, usize)>,
}

impl OverlaysLocation {

    pub fn void() -> Self {
        Self { location: vec![] }
    }

    pub fn from_output(output: &OutputCollector) -> Self {
        Self { 
            location: output.outputs.iter().map(|output| (output.initial_location.index, output.index)).collect() 
        }
    }

    pub fn insert_marker(node: &Node) {
        // insert OVERLAYS_BEGIN_MARKER and OVERLAYS_END_MARKER for all span with class overlay in children of node_ref
        let element = node.dyn_ref::<Element>().expect("Cannot convert to Element");
        let spans = element.get_elements_by_class_name("ghost_overlay");
        for i in 0..spans.length() {
            let span = spans.item(i).expect("Cannot get span");
            let span = span.dyn_ref::<Element>().expect("Cannot convert to Element");

            // get index of id
            let class_name = span.class_name();
            let index = class_name.split("ghost_overlay_").last().expect("Cannot get index").parse::<usize>().expect("Cannot parse index");

            let text = OVERLAYS_BEGIN_MARKER.to_string() + &index.to_string() + &OVERLAYS_END_MARKER.to_string();
            span.set_inner_html(&text);
        }
    }

    pub fn retrieve_overlays_location(text: &str) -> (Self, String) {
        let mut overlays_location = OverlaysLocation{location: vec![]};
        let mut new_string = String::new();
        let mut last_cut_index = 0;
        let mut nb_invalid_characters = 0;
        loop {

            // find start index of ghost begin marker
            let start_index = text[last_cut_index..].find(OVERLAYS_BEGIN_MARKER);
            if start_index.is_none() { break; }
            let start_index = start_index.unwrap() + last_cut_index;

            // find end index of ghost end marker
            let end_index = text[start_index..].find(OVERLAYS_END_MARKER);
            if end_index.is_none() { break; }
            let end_index = end_index.unwrap() + start_index;

            // get index of ghost overlay
            let str_index = &text[start_index+OVERLAYS_BEGIN_MARKER.to_string().len()..end_index];
            let index = str_index.parse::<usize>().expect("Cannot parse index");

            // count number of invalid characters between start and end index
            nb_invalid_characters += text[last_cut_index..start_index]
                .chars()
                .filter(|c| c == &CURSOR_BEGIN_MARKER || c == &CURSOR_END_MARKER)
                .map(|c| c.len_utf8())
                .sum::<usize>();

            // add text to new string
            new_string += &text[last_cut_index..start_index];
            
            overlays_location.location.push((new_string.len() - nb_invalid_characters, index));

            last_cut_index = end_index + OVERLAYS_END_MARKER.to_string().len();
        }
        // add remaining text to new string
        new_string += &text[last_cut_index..];

        (overlays_location, new_string)
    }

    pub fn restore_overlays_location(&self, marking: &mut Marking) {

        for (position, overlay_location) in &self.location {
            marking.insert_overlay(*position, OverlayIndex{index: *overlay_location});
        }
    }
}