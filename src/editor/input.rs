// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use leptos::html;
use leptos::html::Div;
use log::info;

use crate::lexer::*;
use crate::scope::output::*;

use super::cursor::*;
use super::stylization::*;
use super::ghost::*;

fn stylize_text<T: Fn(&str, &mut Stylization, GhostReversePlacement) -> ()>(node_ref: NodeRef<Div>, f: T) {
    // All this function is to avoid cursor disruption when we add style to text

    let node = node_ref.get().expect("Node ref is not a div or not found");

    // insert marker
    CursorState::insert_marker(&node);
    GhostReversePlacement::insert_marker(&node);

    // get text content of node_ref
    let text = node.text_content().unwrap_or_default();

    // get cursor state, ghost reverse placement and text
    let (ghost_reverse_placement, brute_text) = GhostReversePlacement::retrieve_ghost_overlay(&text);
    let (cursor_state, brute_text) = CursorState::retrieve_cursor(&brute_text);

    let mut stylization = Stylization::new();

    // apply function f to text
    f(&brute_text, &mut stylization, ghost_reverse_placement);

    // apply cursor state to stylization
    cursor_state.place_cursor_balise(&mut stylization);

    // apply stylization to text
    let modified_text = stylization.apply_to_text(&brute_text);

    // set innerhtml of node_ref to modified_text
    node.set_inner_html(&modified_text.to_html());

    // replace cursor at original position
    let window = window();
    CursorState::restore_cursor(window);
}

#[component]
pub fn CodeInput(
    #[prop(into)] input_text: RwSignal<String>,
    #[prop(into)] output_signal: RwSignal<OutputCollector>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_run: Callback<String>,
) -> impl IntoView {

    let input_node_ref = NodeRef::<html::Div>::new();
    let output_overlays_node_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {

        // retrieve cursor state and text
        let text = input_text.get();
        let output = output_signal.get();
        let node = input_node_ref.get().expect("Node ref is not a div or not found");
        let output_overlays_node = output_overlays_node_ref.get().expect("Node ref is not a div or not found");

        // verify if text is placed to avoid cursor disruption
        let current_text = node.text_content().unwrap_or_default();
        if current_text != text {
            node.set_inner_html(&text);
        }

        // stylize text
        stylize_text(input_node_ref, |text, stylization, previous_ghost_placement| {

            let lexer = Lexer::new(text);
            lexer.stylize(stylization);

            let ghost_placement = if output.is_placed {
                previous_ghost_placement
            } else {
                output_signal.update(|output| output.is_placed = true);
                GhostReversePlacement::from_output(&output_signal.get())
            };

            ghost_placement.restore_ghost_overlay(stylization);
        });

        // display output overlays
        display_output_overlays(output, output_overlays_node, node);
    });

    let on_run_click = move |_| {
        let text = input_text.get_untracked();
        let text = CursorState::retrieve_cursor(&text).1;
        info!("text: {}", text);
        on_run.run(text);
    };

    view! {
        <div>
            <div
                node_ref=input_node_ref
                class="input"
                contenteditable="true"
                data-absolute-text-position=0
                on:input=move |ev| {

                    if let Some(target) = ev.target() {
                        if let Ok(element) = target.dyn_into::<HtmlElement>() {
                            let text_content = element.text_content().unwrap_or_default();
                            on_change.run(text_content);
                        }
                    }
                }
                on:keydown=move |ev| {
                    if ev.key() == "Enter" {
                        ev.prevent_default(); // prevent <div> or <p>
                        let selection = window().get_selection().expect("Cannot get selection").expect("No selection available");
                        let range = selection.get_range_at(0).expect("Cannot get range");
                    
                        // Insert a text node containing a line break
                        let br = document().create_text_node("\n");
                        let _ = range.insert_node(&br);
                    
                        // Move cursor after the \n
                        let _ = range.set_start_after(&br);
                        let _ = range.set_end_after(&br);
                        let _ = selection.remove_all_ranges();
                        let _ = selection.add_range(&range);
                    }

                    if let Some(target) = ev.target() {
                        if let Ok(element) = target.dyn_into::<HtmlElement>() {
                            let text_content = element.text_content().unwrap_or_default();
                            on_change.run(text_content);
                        }
                    }
                }
                spellcheck="false"
            />
            <div node_ref=output_overlays_node_ref>
            </div>
            <button on:click=on_run_click>"Run"</button>
        </div>
    }
}
