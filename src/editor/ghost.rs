// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use wasm_bindgen::JsCast;
use web_sys::{Element, Node, HtmlDivElement, HtmlElement};
use leptos::prelude::*;

use crate::interpreter::scope::output::*;

use super::stylization::*;
use super::cursor::{CURSOR_BEGIN_MARKER, CURSOR_END_MARKER};

const GHOST_BEGIN_MARKER: char = '\u{e003}';
const GHOST_END_MARKER: char = '\u{e004}';

#[derive(Debug)]
pub struct GhostReversePlacement {
    marker: Vec<(usize, usize)>,
}

impl GhostReversePlacement {

    pub fn from_output(output: &OutputCollector) -> Self {
        Self { 
            marker: output.outputs.iter().map(|output| (output.initial_location.index, output.index)).collect() 
        }
    }

    pub fn insert_marker(node: &Node) {
        // insert GHOST_BEGIN_MARKER and GHOST_END_MARKER for all span with class ghost_overlay in children of node_ref
        let element = node.dyn_ref::<Element>().expect("Cannot convert to Element");
        let spans = element.get_elements_by_class_name("ghost_overlay");
        for i in 0..spans.length() {
            let span = spans.item(i).expect("Cannot get span");
            let span = span.dyn_ref::<Element>().expect("Cannot convert to Element");

            // get index of id
            let class_name = span.class_name();
            let index = class_name.split("ghost_overlay_").last().expect("Cannot get index").parse::<usize>().expect("Cannot parse index");

            let text = GHOST_BEGIN_MARKER.to_string() + &index.to_string() + &GHOST_END_MARKER.to_string();
            span.set_inner_html(&text);
        }
    }

    pub fn retrieve_ghost_overlay(text: &str) -> (Self, String) {
        let mut ghost_reverse_placement = GhostReversePlacement{marker: vec![]};
        let mut new_string = String::new();
        let mut last_cut_index = 0;
        let mut nb_invalid_characters = 0;
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
            let str_index = &text[start_index+GHOST_BEGIN_MARKER.to_string().len()..end_index];
            let index = str_index.parse::<usize>().expect("Cannot parse index");

            // count number of invalid characters between start and end index
            nb_invalid_characters += text[last_cut_index..start_index]
                .chars()
                .filter(|c| c == &CURSOR_BEGIN_MARKER || c == &CURSOR_END_MARKER)
                .map(|c| c.len_utf8())
                .sum::<usize>();

            // add text to new string
            new_string += &text[last_cut_index..start_index];
            
            ghost_reverse_placement.marker.push((new_string.len() - nb_invalid_characters, index));

            last_cut_index = end_index + GHOST_END_MARKER.to_string().len();
        }
        // add remaining text to new string
        new_string += &text[last_cut_index..];

        (ghost_reverse_placement, new_string)
    }

    pub fn restore_ghost_overlay(&self, stylization: &mut Stylization) {

        for (position, ghost_overlay) in &self.marker {
            stylization.insert_ghost_overlay(*position, GhostOverlayIndex{index: *ghost_overlay});
        }
    }
}

#[component]
pub fn GhostOverlayComponent(
    #[prop(into)] index: usize,
    children: Children,
) -> impl IntoView {
    // ghost overlay as goal to display element without insert text or modify text user input
    // make a component in absolutely positionned at the position of the ghost overlay dynamically and set width and height of the span in the text	
    
    view! {
        <div 
            class=format!("output_overlay output_overlay_{}", index)
            style:position="absolute" 
            style:left="0px"
            style:top="0px"
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TextGhostOverlay(
    #[prop(into)] output: Output,
) -> impl IntoView {
    view! {
        <GhostOverlayComponent index=output.index>
            {output.text}
        </GhostOverlayComponent>
    }
}

pub fn render_output_overlays(
    output: &OutputCollector,
) -> impl IntoView {
    let mut view_collection = Vec::new();

    for output in &output.outputs {
        view_collection.push(view! {
            <div>
                <TextGhostOverlay output=output.clone() />
            </div>
        });
    }

    view_collection
}

pub fn display_output_overlays(
    output: OutputCollector,
    output_overlays_node: HtmlDivElement,
    input_node: HtmlDivElement,
) {

    // display output overlays
    output_overlays_node.set_inner_html(&render_output_overlays(&output).to_html());

    let mut elements = Vec::new();
    
    for output in output.outputs {
        let index = output.index;

        // find span element
        let span_element = input_node
            .get_elements_by_class_name(format!("ghost_overlay_{}", index).as_str())
            .item(0).expect("Cannot get output overlay")
            .dyn_into::<HtmlElement>().expect("Cannot convert to HtmlElement");

        // find overlay element
        let overlay_element = output_overlays_node
            .get_elements_by_class_name(format!("output_overlay_{}", index).as_str())
            .item(0).expect("Cannot get output overlay")
            .dyn_into::<HtmlElement>().expect("Cannot convert to HtmlElement");

        // register elements
        elements.push((span_element, overlay_element));
    }

    // first loop to set width of span element
    for (span_element, overlay_element) in &elements {
        
        // get width of overlay element
        let width = overlay_element.client_width();

        // set width of span element
        let style = HtmlElement::style(&span_element);
        style.set_property("width", format!("{}px", width).as_str()).expect("Cannot set width");
    }

    // second loop to set position of overlay element
    for (span_element, overlay_element) in &elements {

        // get position of span element
        let position = span_element.get_bounding_client_rect();

        // set position of overlay element
        let style = HtmlElement::style(&overlay_element);
        style.set_property("left", format!("{}px", position.left()).as_str()).expect("Cannot set left");
        style.set_property("top", format!("{}px", position.top()).as_str()).expect("Cannot set top");
    }
}