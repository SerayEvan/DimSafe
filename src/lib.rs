// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

mod storage;
mod derive_rw;
mod input;
mod stylization;
mod ast;
mod lexer;
mod parser;
mod shownable;
mod interperter;
mod scope;
mod error;

use wasm_bindgen::prelude::*;
use leptos::prelude::*;
use console_error_panic_hook;
use console_log;
use log::Level;

use storage::*;
use input::*;
use parser::*;
use scope::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    // Set panic hook for better error messages in browser console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Erreur lors de l'initialisation du logger");
    
    mount_to_body(|| {

        let signal = local_storage_signal("value".to_string(), Some("".to_string())).unwrap();
        let ast_signal = RwSignal::new(None);
        let on_change = Callback::new(move |value: String| {
            signal.set(value);
        });
        let on_run = Callback::new(move |value: String| {
            let program = parse_program(&value);
            ast_signal.set(Some(program));
        });

        view! {
            <div>
                <h1>"SmartCalc"</h1>
            </div>
            <CodeInput input_text=signal ast_signal=ast_signal on_change=on_change on_run=on_run />
            <footer>
                <p>"© 2025 Evan SERAY — Tous droits réservés"</p>
            </footer>
        }
    });

    Ok(())
}
