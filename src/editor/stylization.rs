// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

/*

When we want to add style to the text, we use a stylization mechanism to add spans to the text.
The sensitive part is to avoid balise miss placement when the text is modified.
The other part is to avoid collision between balises.
To avoid this, we use a map of markers to store the balises.
To avoid collision between balises, we use binary operations to merge the balises.
Finally we use this map at the end to apply the balises to the text.

*/

use leptos::prelude::*;

use std::collections::BTreeMap;
use std::ops::Bound;
use std::cmp::min;

use crate::interpreter::lexer::*;

type Balise = usize;

pub static BALISE_LIST: [&'static str; 9] = [
    "cursor_begin",
    "cursor_end",
    "literal_numerical",
    "literal_keyword",
    "literal_string",
    "literal_unit",
    "identifier",
    "operator",
    "structural",
];

// balise use when we don't want to have a balise
pub const EMPTY_BALISE: Balise = 0;
pub const CURSOR_BEGIN: Balise = 1 << 0;
pub const CURSOR_END: Balise = 1 << 1;
pub const LITERAL_NUMERICAL_BALISE: Balise = 1 << 2;
pub const LITERAL_KEYWORD_BALISE: Balise = 1 << 3;
pub const LITERAL_STRING_BALISE: Balise = 1 << 4;
pub const LITERAL_UNIT_BALISE: Balise = 1 << 5;
pub const IDENTIFIER_BALISE: Balise = 1 << 6;
pub const OPERATOR_BALISE: Balise = 1 << 7;
pub const STRUCTURAL_BALISE: Balise = 1 << 8;

fn get_balise_class(balise: Balise) -> String {
    let mut class = String::new();
    for i in 0..BALISE_LIST.len() {
        if balise & (1 << i) != 0 {
            class.push_str(BALISE_LIST[i]);
            class.push_str(" ");
        }
    }
    class.pop();
    class
}

pub fn get_balise(token: &Token) -> usize {
    match token {
        Token::LiteralNumerical(_) => LITERAL_NUMERICAL_BALISE,
        Token::LiteralKeyword(_) => LITERAL_KEYWORD_BALISE,
        Token::LiteralString(_) => LITERAL_STRING_BALISE,
        Token::LiteralUnit(_) => LITERAL_UNIT_BALISE,
        Token::Identifier(_) => IDENTIFIER_BALISE,
        Token::StructuralToken(_) => STRUCTURAL_BALISE,
        Token::AssignmentOperator(_) => STRUCTURAL_BALISE,
        Token::AdditiveOperator(_) => OPERATOR_BALISE,
        Token::SubtractiveOperator(_) => OPERATOR_BALISE,
        Token::MultiplicativeOperator(_) => OPERATOR_BALISE,
        Token::PowerOperator(_) => OPERATOR_BALISE,
        Token::VectorMultiplicativeOperator(_) => OPERATOR_BALISE,
        Token::BooleanOperator(_) => OPERATOR_BALISE,
        Token::ComparatorOperator(_) => OPERATOR_BALISE,
        Token::RawError => EMPTY_BALISE,
    }
}


#[derive(Clone,Copy,Debug)]
pub struct GhostOverlayIndex {
    pub index: usize,
}

#[derive(Clone,Default)]
struct Marker {
    balise: Balise,
    ghost_overlays: Vec<GhostOverlayIndex>,
}

pub struct Stylization {
    markers: BTreeMap<usize, Marker>, // (position, balise)
}

impl Stylization {

    pub fn new() -> Stylization {

        let mut markers = BTreeMap::new();
        markers.insert(0, Marker::default());

        Stylization {
            markers,
        }
    }

    fn get_or_add_marker(&mut self, position: usize) -> &mut Marker {

        // verify if position is already in the map
        if self.markers.contains_key(&position) {
            return self.markers.get_mut(&position).expect("");
        }

        // get previous marker balise
        let previous_marker_balise = self.markers
            .range((Bound::Included(&0), Bound::Included(&position)))
            .last()
            .map(|(_, marker)| marker.balise)
            .unwrap_or(EMPTY_BALISE);
    
        // create new marker
        let new_marker = Marker {
            balise: previous_marker_balise,
            ghost_overlays: vec![],
        };

        // add new marker
        self.markers.insert(position, new_marker);

        // return the nex marker
        self.markers.get_mut(&position).expect("")
    }

    pub fn insert_ghost_overlay(&mut self, position: usize, ghost_overlay: GhostOverlayIndex) {

        // get or add marker to insert ghost overlay
        let marker = self.get_or_add_marker(position);

        // add ghost overlay to marker
        marker.ghost_overlays.push(ghost_overlay);
    }

    pub fn insert_balise(&mut self, new_balise: Balise, position: (usize, usize)) {

        // verify if position is valid
        if position.0 >= position.1 {
            return;
        }

        // verify if position is already in the map
        self.get_or_add_marker(position.0);
        self.get_or_add_marker(position.1);

        // add new balise to overlaped balises
        let inner_balises = self.markers.range_mut(
            (
                Bound::Included(&position.0), 
                Bound::Excluded(&position.1)
            )
        );

        for (_, marker) in inner_balises {
            marker.balise = marker.balise | new_balise;
        }
    }

    pub fn apply_to_text(&self, text: &str) -> impl IntoView {

        let mut view_collection = Vec::new();

        let mut markers_it = self.markers.iter().peekable();
        
        while let Some((pos, marker)) = markers_it.next() {

            // get next marker position
            let next_pos = *markers_it.peek().unwrap_or(&(&text.len(),&Marker::default())).0;

            // get section boundaries
            let (pos, next_pos) = (min(*pos, text.len()), min(next_pos, text.len()));

            // Find valid UTF-8 character boundaries by incrementing if needed
            let mut start_byte = pos;
            while start_byte < text.len() && !text.is_char_boundary(start_byte) {
                start_byte += 1;
            }

            let mut end_byte = next_pos;
            while end_byte < text.len() && !text.is_char_boundary(end_byte) {
                end_byte += 1;
            }

            // add ghost overlay to new text
            for ghost_overlay in marker.ghost_overlays.iter() {
                view_collection.push(view! {
                    <span 
                        class=format!("ghost_overlay ghost_overlay_{}", ghost_overlay.index)
                    ></span>
                }.into_any());
            }


            if start_byte < end_byte {
                // add section to new text with balise class
                view_collection.push(view! {
                    <span class={get_balise_class(marker.balise)}>
                        {&text[start_byte..end_byte]}
                    </span>
                }.into_any());

            }
            else {
                // even if the text is empty, we need to use this if statement to avoid leptos to add a span instead of nothing
                view_collection.push(view! {
                    <span class={get_balise_class(marker.balise)}/>
                }.into_any());
            }
        }
        
        view_collection
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_get_balise_class() {

        let test_text = "1+2*32 ";

        let mut stylization = Stylization::new();

        // cursor
        stylization.insert_balise(CURSOR_BEGIN, (2, 3));
        stylization.insert_balise(CURSOR_END, (2, 3));

        // highlight text
        stylization.insert_balise(LITERAL_NUMERICAL_BALISE, (0, 1));
        stylization.insert_balise(OPERATOR_BALISE, (1, 2));
        stylization.insert_balise(LITERAL_NUMERICAL_BALISE, (2, 3));
        stylization.insert_balise(OPERATOR_BALISE, (3, 4));
        stylization.insert_balise(LITERAL_NUMERICAL_BALISE, (4, 6));

        // apply text style
        let style_text = stylization.apply_to_text(&test_text).to_html();

        // assert style text
        assert_eq!(style_text, "\
        <span class=\"literal_numerical\">1</span>\
        <span class=\"operator\">+</span>\
        <span class=\"cursor_begin cursor_end literal_numerical\">2</span>\
        <span class=\"operator\">*</span>\
        <span class=\"literal_numerical\">32</span>\
        <span class=\"\"> </span><!>");
    }

    #[test]
    fn test_with_multiple_ghost_overlay() {

        let test_text = "Un peu de texte";

        let mut stylization = Stylization::new();

        let ghost_overlay_0 = GhostOverlayIndex{index: 0};
        let ghost_overlay_1 = GhostOverlayIndex{index: 1};

        // ghost overlay
        stylization.insert_ghost_overlay(4, ghost_overlay_0);
        stylization.insert_ghost_overlay(4, ghost_overlay_1);

        // apply text style
        let style_text = stylization.apply_to_text(&test_text).to_html();

        // assert style text
        assert_eq!(style_text, "\
        <span class=\"\">Un p</span>\
        <span class=\"ghost_overlay ghost_overlay_0\"></span>\
        <span class=\"ghost_overlay ghost_overlay_1\"></span>\
        <span class=\"\">eu de texte</span><!>");

    }
}