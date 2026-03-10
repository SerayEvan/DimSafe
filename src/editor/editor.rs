// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::*;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use leptos::html;

use crate::interpreter::execute::*;

use super::cursor::*;
use super::editor_pipeline::*;
use super::line_indicator::*;

#[component]
pub fn EditorSection(
    #[prop(into)] input_text: RwSignal<String>,
    #[prop(into)] execute_result_signal: RwSignal<ProgramResult>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_run: Callback<String>,
) -> impl IntoView {

    let input_node_ref = NodeRef::<html::Div>::new();
    let output_overlays_node_ref = NodeRef::<html::Div>::new();
    let line_indicator_node_ref = NodeRef::<html::Div>::new();

    let editor_pipeline = EditorPipeline {
        input_node_ref,
        output_overlays_node_ref,
        line_indicator_node_ref,
        input_text,
        execute_result_signal,
    };
    editor_pipeline.piplining_effect();

    view! {
        <div class="editor_section">
            <button 
                on:click=move |_| {
                    let text = input_text.get_untracked();
                    let text = CursorState::retrieve_cursor(&text).1;
                    on_run.run(text);
                }
            >
                "Run"
            </button>
            <div class="editable_area">
                <LineIndicator node=line_indicator_node_ref />
                <div class="input_and_output_container">
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
                            handle_enter_keydown(ev, on_change);
                        }
                    />
                    <div 
                        style:position="relative"
                        node_ref=output_overlays_node_ref
                    >
                    </div>
                </div>
            </div>
        </div>
    }
}
