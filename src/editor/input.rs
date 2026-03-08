// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use leptos::html;

use crate::interpreter::execute::*;

use super::cursor::*;
use super::input_pipeline::InputPipeline;

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

    let input_pipeline = InputPipeline {
        input_node_ref,
        output_overlays_node_ref,
        line_indicator_node_ref,
        input_text,
        execute_result_signal,
    };
    input_pipeline.piplining_effect();

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
                    style:overflow-y="scroll"
                    style:width="100%"
                >
                    <div
                        node_ref=input_node_ref
                        class="input"
                        contenteditable="true"
                        spellcheck="false"

                        on:input=move |ev| {

                            if let Some(target) = ev.target() {
                                if let Ok(element) = target.dyn_into::<HtmlElement>() {

                                    execute_result_signal.set(ProgramResult::Unexecuted);
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
            </div>
        </div>
    }
}
