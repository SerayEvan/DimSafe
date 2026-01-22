// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

mod editor;
mod interpreter;

use wasm_bindgen::prelude::*;
use leptos::prelude::*;
use console_error_panic_hook;
use console_log;
use log::Level;
use log::info;

use editor::input::*;
use editor::storage::*;
use interpreter::parser::*;
use interpreter::scope::*;
use interpreter::error::collector::*;
use interpreter::scope::output::*;
use interpreter::ast::ast_node::*;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {

    // Set panic hook for better error messages in browser console
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).expect("Erreur lors de l'initialisation du logger");
    
    mount_to_body(|| {

        let signal = local_storage_signal("value".to_string(), Some("".to_string())).unwrap();
        let output_signal = RwSignal::new(OutputCollector::new());
        let is_executed = RwSignal::new(false);
        let on_change = Callback::new(move |value: String| {
            signal.set(value);
        });
        let on_run = Callback::new(move |value: String| {
            let program = parse_program(&value);
            if let Ok(program) = program {
                let mut scope = Scope::new();
                let mut errors = ErrorCollector::new();
                let mut output = OutputCollector::new();
                let _result = program.evaluate(&mut scope, &mut errors, &mut output);
                info!("{}", errors.into_string());
                is_executed.set(true);
                output_signal.set(output);
            }
        });

        view! {
            <div>
                <h1>"DimSafe"</h1>
            </div>
            <CodeInput input_text=signal output_signal=output_signal is_executed=is_executed on_change=on_change on_run=on_run />
            <footer>
                <p>
                    "© 2025 Evan SERAY | "
                    <a href="https://github.com" target="_blank" rel="noopener noreferrer">"GitHub"</a>
                </p>
            </footer>
        }
    });

    Ok(())
}
