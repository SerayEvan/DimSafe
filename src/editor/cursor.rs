// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

/*

The user cursor is a sensitive element of the editor, it can be disrupted by the addition of style to the text.

To avoid this disruption, we use three steps:
1) We use markers (invalid unicode characters) to mark the cursor position in the text.
2) We retrieve the cursor position from the text and remove the markers.
3) With stylization mechanism we add spans to the text to mark the cursor position.
4) We place the cursor at the correct position in the text with spans balises.

*/

use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Window, Element, Node};

use super::stylization::*;

pub const CURSOR_BEGIN_MARKER: char = '\u{e001}';
pub const CURSOR_END_MARKER: char = '\u{e002}';

pub struct CursorState {
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

    pub fn insert_marker(node: &Node) {

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

            // verify if start_node and end_node are children of node
            if !node.contains(Some(&start_node)) || !node.contains(Some(&end_node)) {
                continue;
            }

            // insert marker at end of range
            insert_marker(&end_node, end_offset, &CURSOR_END_MARKER.to_string());
            insert_marker(&start_node, start_offset, &CURSOR_BEGIN_MARKER.to_string());
        }
    }

    pub fn retrieve_cursor(text: &str) -> (CursorState, String) {

        /* WARNING: This function did not work as expected if the text contains other markers that will be erased
        by retrieve_ghost_overlay because he changes position of the markers in the text.
        So you need to call this function after retrieve_ghost_overlay */
        
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
            remove_counter += CURSOR_BEGIN_MARKER.to_string().len();
            last_cut_index = start_index+remove_counter;

            let end_index = text[new_string.len() + remove_counter..].find(CURSOR_END_MARKER);
            if end_index.is_none() { break; }
            let end_index = end_index.unwrap() + new_string.len();
            new_string += &text[last_cut_index..end_index+remove_counter];
            remove_counter += CURSOR_END_MARKER.to_string().len();
            last_cut_index = end_index+remove_counter;

            cursor_state.cursor.push([start_index, end_index]);
        }
        new_string += &text[last_cut_index..];
        (cursor_state, new_string)
    }

    pub fn place_cursor_balise(&self, stylization: &mut Stylization) {
        for cursor in &self.cursor {
            let [cursor_begin, cursor_end] = *cursor;
            stylization.insert_balise(CURSOR_BEGIN, (cursor_begin, cursor_begin + 1));
            stylization.insert_balise(CURSOR_END, (cursor_end, cursor_end + 1));
        }
    }

    pub fn restore_cursor(window: Window) {

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