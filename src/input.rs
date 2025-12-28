// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Element, Node, HtmlElement};
use leptos::html;
use log::info;

use crate::lexer::*;
use crate::stylization::*;
use crate::shownable::*;

const CURSOR_BEGIN_MARKER: &str = "\u{e001}";
const CURSOR_END_MARKER: &str = "\u{e002}";

struct CursorState {
    cursor: Vec<[usize; 2]>,
}

fn insert_marker(parent_node: &Node, offset: u32, marker: &str) {

    // Create a Range object from the document owning the parent_node
    let document = parent_node
        .owner_document()
        .expect("Cannot get owner document");
    let range = document
        .create_range()
        .expect("Cannot create range");

    // Set the start and end of the range at the given offset in the parent_node
    range
        .set_start(parent_node, offset)
        .expect("Cannot set start");
    range
        .set_end(parent_node, offset)
        .expect("Cannot set end");

    // Create a text node for the marker
    let marker_node = document
        .create_text_node(marker)
        .dyn_into::<Node>()
        .expect("Cannot convert to Node");

    // Insert the marker node at the range
    range
        .insert_node(&marker_node)
        .expect("Cannot insert marker");
}

impl CursorState {

    fn insert_marker() {

        let window = window();
        
        let selections = window.get_selection()
                                          .expect("Cannot get selection")
                                          .expect("Cannot get selection");
        
        for i in 0..selections.range_count() {

            // add a text marker at begin and end of the range
            let range = selections.get_range_at(i).expect("Cannot get range");

            // use two non-assigned unicode "Private Use Areas" characters to mark the cursor position
            let start_node = range.start_container().expect("Cannot get start container");
            let start_offset = range.start_offset().expect("Cannot get start offset");
            let end_node = range.end_container().expect("Cannot get end container");
            let end_offset = range.end_offset().expect("Cannot get end offset");
            insert_marker(&end_node, end_offset, CURSOR_END_MARKER);
            insert_marker(&start_node, start_offset, CURSOR_BEGIN_MARKER);
        }
    }

    fn retrieve_cursor(text: &str) -> (CursorState, String) {
        // retrieve cursor position from string and remove markers
        let mut cursor_state = CursorState{cursor: vec![]};
        let mut new_string = String::new();
        let mut remove_counter = 0;
        let mut last_cut_index = 0;
        loop {

            let start_index = text[new_string.len() + remove_counter..].find(CURSOR_BEGIN_MARKER);
            if start_index.is_none() { break; }
            let start_index = start_index.unwrap() + new_string.len();
            new_string += &text[last_cut_index..start_index+remove_counter];
            remove_counter += CURSOR_BEGIN_MARKER.len();
            last_cut_index = start_index+remove_counter;

            let end_index = text[new_string.len() + remove_counter..].find(CURSOR_END_MARKER);
            if end_index.is_none() { break; }
            let end_index = end_index.unwrap() + new_string.len();
            new_string += &text[last_cut_index..end_index+remove_counter];
            remove_counter += CURSOR_END_MARKER.len();
            last_cut_index = end_index+remove_counter;

            cursor_state.cursor.push([start_index, end_index]);
        }
        new_string += &text[last_cut_index..];
        (cursor_state, new_string)
    }

    fn place_cursor_balise(&self, stylization: &mut Stylization) {
        for cursor in &self.cursor {
            let [cursor_begin, cursor_end] = *cursor;
            stylization.insert_balise(CURSOR_BEGIN, (cursor_begin, cursor_begin + 1));
            stylization.insert_balise(CURSOR_END, (cursor_end, cursor_end + 1));
        }
    }

    fn set_cursor(window: Window) {

        // catch node cursor begin and cursor end
        let document = window.document().expect("Cannot get document");
        let node_begin = document
            .query_selector_all(".cursor_begin")
            .expect("Cannot query selector");
        let node_end = document
            .query_selector_all(".cursor_end")
            .expect("Cannot query selector");

        for i in 0..u32::min(node_begin.length(), node_end.length()) {

            let node_begin = node_begin.item(i).expect("Cannot get node begin");
            let node_end = node_end.item(i).expect("Cannot get node end");

            // Get the global selection object
            let selection = window
                .get_selection()
                .expect("Cannot get selection")
                .expect("No selection available");

            // Create a new Range
            let range = document
                .create_range()
                .expect("Cannot create range");

            // Set the start of the range at the beginning of node_begin
            range.set_start(&node_begin, 0).expect("Failed to set start");
            range.set_end(&node_end, 0).expect("Failed to set end");

            // Remove all existing ranges and add the new one
            selection.remove_all_ranges().expect("Failed to remove all ranges");
            selection.add_range(&range).expect("Failed to add range");

            // remove class "cursor_begin" and "cursor_end" from node_begin and node_end
            if let Some(elt) = &node_begin.dyn_ref::<Element>() {
                let _ = elt.class_list().remove_1("cursor_begin");
            }
            if let Some(elt) = &node_end.dyn_ref::<Element>() {
                let _ = elt.class_list().remove_1("cursor_end");
            }
        }
    }
}

#[component]
pub fn CodeInput(
    //cx: Scope,
    #[prop(into)] input_text: RwSignal<String>,
    #[prop(into)] ast_signal: RwSignal<impl Shownable + Clone>,
    #[prop(into)] on_change: Callback<String>,
    #[prop(into)] on_run: Callback<String>,
) -> impl IntoView {

    let input_node_ref = NodeRef::<html::Div>::new();

    Effect::new(move |_| {

        // get cursor state
        let window = window();

        // retrieve cursor state and text
        let brute_text_with_cursor_marker = input_text.get();
        let (cursor_state, brute_text) = CursorState::retrieve_cursor(&brute_text_with_cursor_marker);

        // lexer text
        let lexer = Lexer::new(&brute_text);

        // apply cursor balise
        let mut style_effect = Stylization::new();
        cursor_state.place_cursor_balise(&mut style_effect);
        lexer.stylize(&mut style_effect);

        // process text style
        let style_text = style_effect.apply_to_text(&brute_text);

        // set innerhtml of python-input to value.get()
        if let Some(node) = input_node_ref.get() {
            node.set_inner_html(&style_text);
        }

        // replace cursor at original position
        CursorState::set_cursor(window);
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
                            CursorState::insert_marker();
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
                            CursorState::insert_marker();
                            let text_content = element.text_content().unwrap_or_default();
                            on_change.run(text_content);
                        }
                    }
                }
                spellcheck="false"
            />
            <button on:click=on_run_click>"Run"</button>
            <div class="output">
                {move || ast_signal.read().clone().display()}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_retrieve_cursor() {
        let string = "1+2*3\u{E001}4\u{E002}";
        let (cursor_state, string) = CursorState::retrieve_cursor(string);
        assert_eq!(cursor_state.cursor, vec![[5, 6]]);
        assert_eq!(string, "1+2*34");
    }

    #[test]
    fn test_get_retrieve_cursor2() {
        let string = "1+2*3\u{E001}\u{E002}4";
        let (cursor_state, string) = CursorState::retrieve_cursor(string);
        assert_eq!(cursor_state.cursor, vec![[5, 5]]);
        assert_eq!(string, "1+2*34");
    }
}
