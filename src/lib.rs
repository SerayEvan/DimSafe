// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

mod editor;
mod interpreter;

use wasm_bindgen::prelude::*;
use leptos::prelude::*;
use console_error_panic_hook;
use console_log;
use log::Level;

use editor::editor::*;
use editor::storage::*;
use interpreter::execute::*;
use interpreter::scope::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    // Set panic hook for better error messages in browser console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Erreur lors de l'initialisation du logger");
    
    leptos::mount::mount_to_body(|| {

        let signal = local_storage_signal("value".to_string(), Some("".to_string())).unwrap();
        let execute_result_signal = RwSignal::new(ProgramResult::Unexecuted);

        let on_change = Callback::new(move |value: String| {
            signal.set(value);
        });

        let on_run = Callback::new(move |source: String| {
            let mut scope = Scope::new();
            execute_result_signal.set(execute_program(&mut scope, &source));
        });

        view! {
            <div>
                <h1>"DimSafe"</h1>
            </div>
            <EditorSection input_text=signal execute_result_signal=execute_result_signal on_change=on_change on_run=on_run />
            <footer>
                <p>
                    "© 2025 Evan SERAY | "
                    <a href="https://github.com/SerayEvan/DimSafe" target="_blank" rel="noopener noreferrer">"GitHub"</a>
                </p>
            </footer>
        }
    });

    Ok(())
}
