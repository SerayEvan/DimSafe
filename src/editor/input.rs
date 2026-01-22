// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use leptos::html;
use leptos::html::Div;

use crate::interpreter::lexer::*;
use crate::interpreter::scope::output::*;

use super::cursor::*;
use super::stylization::*;
use super::ghost::*;

fn stylize_text<T: Fn(&str, &mut Stylization, GhostReversePlacement) -> ()>(node_ref: NodeRef<Div>, f: T) {
    // All this function is to avoid cursor disruption when we add style to text

    let node = node_ref.get().expect("Node ref is not a div or not found");

    let text = node.text_content().unwrap_or_default();

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
    let html_text = modified_text.to_html();
    node.set_inner_html(&html_text);

    // replace cursor at original position
    let window = window();
    CursorState::restore_cursor(window);
}

fn stylize_text_with_lexer(text: &str, stylization: &mut Stylization) {
    let lexer = Lexer::new(text);
    for token in lexer {
        if let Ok((start, token, end)) = token {
            stylization.insert_balise(get_balise(&token), (start, end));
        }
    }
}

fn set_line_indicator(node: &NodeRef<Div>, text: &str) {
    let lines = text.chars().filter(|c| *c == '\n').count() + 1;
    let mut inner_view = Vec::new();
    for line in 0..lines {
        inner_view.push(view! {
            <div>
                {format!("{}", line + 1)}
            </div>
        });
    }
    let html_text = inner_view.into_iter().collect::<Vec<_>>().to_html();
    node.get_untracked().expect("Node ref is not a div or not found").set_inner_html(&html_text);
}

#[component]
pub fn CodeInput(
    #[prop(into)] input_text: RwSignal<String>,
    #[prop(into)] output_signal: RwSignal<OutputCollector>,
    #[prop(into)] is_executed: RwSignal<bool>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_run: Callback<String>,
) -> impl IntoView {

    let input_node_ref = NodeRef::<html::Div>::new();
    let output_overlays_node_ref = NodeRef::<html::Div>::new();
    let line_indicator_node_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {

        // retrieve cursor state and text
        let text = input_text.get();
        let output = output_signal.get();
        let node = input_node_ref.get_untracked().expect("Node ref is not a div or not found");
        let output_overlays_node = output_overlays_node_ref.get_untracked().expect("Node ref is not a div or not found");

        // verify if text is placed to avoid cursor disruption
        let current_text = node.text_content().unwrap_or_default();
        if current_text != text {
            node.set_inner_html(&text);
        }

        // stylize text
        stylize_text(input_node_ref, |text, stylization, previous_ghost_placement| {

            stylize_text_with_lexer(text, stylization);

            let ghost_placement = if !is_executed.get_untracked() {
                previous_ghost_placement
            } else {
                GhostReversePlacement::from_output(&output_signal.get())
            };
            is_executed.set(false);

            ghost_placement.restore_ghost_overlay(stylization);
        });

        // display output overlays
        display_output_overlays(output, output_overlays_node, node);

        // set line indicator
        set_line_indicator(&line_indicator_node_ref, &text);
    });

    let on_run_click = move |_| {
        let text = input_text.get_untracked();
        let text = CursorState::retrieve_cursor(&text).1;
        on_run.run(text);
    };

    view! {
        <button on:click=on_run_click>"Run"</button>
        <div class="box_input">
            <div 
                node_ref=line_indicator_node_ref
                class="line_number"
            />
            <div
                node_ref=input_node_ref
                class="input"
                contenteditable="true"
                spellcheck="false"

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
                        if let Some(target) = ev.target() {
                            if let Ok(element) = target.dyn_into::<HtmlElement>() {
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

                                // call input event
                                let text_content = element.text_content().unwrap_or_default();
                                on_change.run(text_content);
                            }
                        }
                    }
                }
            />
            <div node_ref=output_overlays_node_ref>
            </div>
        </div>
    }
}
