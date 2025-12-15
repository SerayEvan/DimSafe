// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use std::collections::BTreeMap;
use std::ops::Bound;
use std::cmp::min;

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

pub struct Stylization {
    balise: BTreeMap<usize, Balise>, // (position, balise)
    
}

impl Stylization {

    pub fn new() -> Stylization {

        let mut balise = BTreeMap::new();
        balise.insert(0, EMPTY_BALISE);

        Stylization {
            balise,
        }
    }

    pub fn insert_balise(&mut self, new_balise: Balise, position: (usize, usize)) {

        // verify if position is valid
        if position.0 >= position.1 {
            return;
        }

        // Get previous balise using lower_bound
        let previous_open_balise = *self.balise
            .range((Bound::Included(&0), Bound::Included(&position.0)))
            .last()
            .unwrap_or((&0, &EMPTY_BALISE))
            .1;

        let previous_close_balise = *self.balise
            .range((Bound::Included(&0), Bound::Included(&position.1)))
            .last()
            .unwrap_or((&0, &EMPTY_BALISE))
            .1;

        // overlape balises
        let inner_balises = self.balise.range_mut((Bound::Excluded(&position.0), Bound::Excluded(&position.1)));
        for (_, balise) in inner_balises {
            *balise = *balise | new_balise;
        }

        // Add new balises
        self.balise.insert(position.0, previous_open_balise | new_balise);
        self.balise.insert(position.1, previous_close_balise);
    }

    pub fn apply_to_text(&self, text: &str) -> String {
        
        let mut new_text = String::new();

        let mut balises = self.balise.iter().peekable();
        
        while let Some((pos, balise)) = balises.next() {
            let next_pos = *balises.peek().unwrap_or(&(&text.len(),&0)).0;

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

            new_text.push_str(&format!(
                "<span class=\"{}\">{}</span>", 
                get_balise_class(*balise),
                &text[start_byte..end_byte]
            ));  
        }
        new_text
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
        let style_text = stylization.apply_to_text(&test_text);
        assert_eq!(style_text, "\
        <span class=\"literal_numerical\">1</span>\
        <span class=\"operator\">+</span>\
        <span class=\"cursor_begin cursor_end literal_numerical\">2</span>\
        <span class=\"operator\">*</span>\
        <span class=\"literal_numerical\">32</span>\
        <span class=\"\"> </span>");
    }
}