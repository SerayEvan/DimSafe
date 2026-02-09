// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use leptos::html;
use leptos::html::Div;
use leptos_use::use_element_bounding;
use log::info;

use crate::interpreter::lexer::*;
use crate::interpreter::execute::*;

use super::cursor::*;
use super::stylization::*;
use super::ghost::*;

fn stylize_text_with_lexer(text: &str, stylization: &mut Stylization) {
    let lexer = Lexer::new(text);
    for token in lexer {
        match token {
            Ok((start, token, end)) => {
                stylization.insert_balise(get_balise(&token), (start, end));
            }
            Err((start, _, end)) => {
                stylization.insert_balise(INVALID_CHARACTER_BALISE, (start, end));
            }
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

struct InputWrapper {
    pub input_node_ref: NodeRef<Div>,
    pub output_overlays_node_ref: NodeRef<Div>,
    pub line_indicator_node_ref: NodeRef<Div>,
    pub input_text: RwSignal<String>,
    pub execute_result_signal: RwSignal<ProgramResult>,
}

impl InputWrapper {

    pub fn piplining_effect(self) {

        Effect::new(move |_| {


            let elt_bounding_signal = use_element_bounding(self.input_node_ref.get().expect("Node ref is not a div or not found"));

            // effect to repaint output overlays when input area is resized and location of output overlays is changed
            Effect::new(move |_| {
                let _ = elt_bounding_signal.width.get();
                let _ = elt_bounding_signal.height.get();
                let _ = elt_bounding_signal.left.get();
                let _ = elt_bounding_signal.top.get();

                update_location_output_overlays(
                    &self.output_overlays_node_ref.get_untracked().expect("Node ref is not a div or not found"), 
                    &self.input_node_ref.get_untracked().expect("Node ref is not a div or not found")
                );
            });
        });

        // effect to piplining when input text is changed
        Effect::new(move |_| {
            self.piplining();
        });
    }

    pub fn piplining(&self) {

        info!("piplining");

        // trigger piplining when input text is changed or execute result is changed
        let new_text = self.input_text.get();
        let execute_result = self.execute_result_signal.get();

        // retrieve nodes
        let node = self.input_node_ref.get_untracked().expect("Node ref is not a div or not found");
        let output_overlays_node = self.output_overlays_node_ref.get_untracked().expect("Node ref is not a div or not found");

        // insert marker
        CursorState::insert_marker(&node);
        GhostReversePlacement::insert_marker(&node);

        // retrieve text
        let plain_text = node.text_content().unwrap_or_default();

        // get cursor state, ghost reverse placement and text
        let (mut ghost_placement, plain_text) = GhostReversePlacement::retrieve_ghost_overlay(&plain_text);
        let (mut cursor_state, cleaned_text) = CursorState::retrieve_cursor(&plain_text);

        // initialize stylization
        let mut stylization = Stylization::new();

        // verify if text need to be modified
        let current_text = if cleaned_text != new_text {
            cursor_state = CursorState::void();
            ghost_placement = GhostReversePlacement::void();
            new_text.to_string()
        } else {
            cleaned_text
        };

        // apply lexer to text
        stylize_text_with_lexer(&current_text, &mut stylization);

        // apply ghost placement from output if executed. Otherwise, keep the previous ghost placement.
        match execute_result {
            ProgramResult::Ok{ output, .. } => {
                ghost_placement = GhostReversePlacement::from_output(&output);
                insert_output_overlays(&output, &output_overlays_node);

                // remove unexecuted class and add executed class to output_overlays_node
                let _ = output_overlays_node
                    .class_list()
                    .remove_1("unexecuted");

                let _ = output_overlays_node
                    .class_list()
                    .add_1("executed");
            }
            ProgramResult::InvalidTokens(e) => {
                ghost_placement = GhostReversePlacement::void();
            }
            ProgramResult::Unexecuted => {
                // add unexecuted class to output_overlays_node and remove executed class
                let _ = output_overlays_node
                    .class_list()
                    .add_1("unexecuted");

                let _ = output_overlays_node
                    .class_list()
                    .remove_1("executed");
            }
        };

        // apply ghost placement to stylization
        ghost_placement.restore_ghost_overlay(&mut stylization);

        // apply cursor state to stylization
        cursor_state.place_cursor_balise(&mut stylization);

        // apply stylization to text
        let modified_text = stylization.apply_to_text(&current_text);

        // set innerhtml of node_ref to modified_text
        let html_text = modified_text.to_html();
        node.set_inner_html(&html_text);

        // replace cursor at original position
        let window = window();
        CursorState::restore_cursor(window);

        // display output overlays
        update_location_output_overlays(&output_overlays_node, &node);

        // set line indicator
        set_line_indicator(&self.line_indicator_node_ref, &current_text);
    }
}

#[component]
pub fn CodeInput(
    #[prop(into)] input_text: RwSignal<String>,
    #[prop(into)] execute_result_signal: RwSignal<ProgramResult>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_run: Callback<String>,
) -> impl IntoView {

    let input_node_ref = NodeRef::<html::Div>::new();
    let output_overlays_node_ref = NodeRef::<html::Div>::new();
    let line_indicator_node_ref = NodeRef::<html::Div>::new();

    let input_wrapper = InputWrapper {
        input_node_ref,
        output_overlays_node_ref,
        line_indicator_node_ref,
        input_text,
        execute_result_signal,
    };
    input_wrapper.piplining_effect();

    view! {
        <div class="input_section">
            <button 
                on:click=move |_| {
                    let text = input_text.get_untracked();
                    let text = CursorState::retrieve_cursor(&text).1;
                    on_run.run(text);
                }
            >
                "Run"
            </button>
            <div class="input_area">
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

                                //batch(move || {
                                    execute_result_signal.set(ProgramResult::Unexecuted);
                                    let text_content = element.text_content().unwrap_or_default();
                                    on_change.run(text_content);
                                //});
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
        </div>
    }
}
