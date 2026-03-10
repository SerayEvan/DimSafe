// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::*;
use leptos::prelude::*;
use leptos::html::Div;

#[component]
pub fn LineIndicator(
    #[prop(into)] node: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div 
            node_ref=node
            class="line_number"
        />
    }
}

pub fn set_line_indicator(node: &NodeRef<Div>, text: &str) {
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