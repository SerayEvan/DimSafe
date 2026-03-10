// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, HtmlElement};
use leptos::*;
use leptos::prelude::*;

use crate::interpreter::scope::output::*;



#[component]
fn GhostOverlayComponent(
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
fn TextGhostOverlay(
    #[prop(into)] output: Output,
) -> impl IntoView {
    match output.message {
        OutputMessage::Result(text) => view! {
            <GhostOverlayComponent index=output.index>
                <span class="output_message output_message_result">
                    {text}
                </span>
            </GhostOverlayComponent>
        }.into_any(),
        OutputMessage::Error(text) => view! {
            <GhostOverlayComponent index=output.index>
                <span class="output_message output_message_error">
                    {text}
                </span>
            </GhostOverlayComponent>
        }.into_any(),
    }
}

fn render_output_overlays(
    output: &OutputCollector,
) -> impl IntoView {
    let mut view_collection = Vec::new();

    for output in &output.outputs {
        view_collection.push(view! {
            <TextGhostOverlay output=output.clone() />
        });
    }

    view_collection
}

pub fn insert_output_overlays(
    output: &OutputCollector,
    output_overlays_node: &HtmlDivElement,
) {
    output_overlays_node.set_inner_html(&render_output_overlays(&output).to_html());
}

pub fn update_location_output_overlays(
    output_overlays_node: &HtmlDivElement,
    input_node: &HtmlDivElement,
) {
    let mut elements = Vec::new();

    let mut i = 0;

    let overlays = output_overlays_node.get_elements_by_class_name("output_overlay");
    
    while let Some(overlay_element) = overlays.item(i) {

        let index = overlay_element.class_name().split("output_overlay_").last().expect("Cannot get index").parse::<usize>().expect("Cannot parse index");

        // find span element
        let span_element = input_node
            .get_elements_by_class_name(format!("ghost_overlay_{}", index).as_str())
            .item(0);

        // verify if span and overlay element are found and continue if not
        match span_element {
            Some(span_element) => {

                // convert span and overlay element to HtmlElement
                let span_element = span_element.dyn_into::<HtmlElement>().expect("Cannot convert to HtmlElement");
                let overlay_element = overlay_element.dyn_into::<HtmlElement>().expect("Cannot convert to HtmlElement");

                // register elements
                elements.push((span_element, overlay_element));
            }
            _ => {}
        }

        i += 1;
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
        let origine = output_overlays_node.get_bounding_client_rect();

        // set position of overlay element
        let style = HtmlElement::style(&overlay_element);
        style.set_property("left", format!("{}px", position.left() - origine.left()).as_str()).expect("Cannot set left");
        style.set_property("top", format!("{}px", position.top() - origine.top()).as_str()).expect("Cannot set top");
    }
}