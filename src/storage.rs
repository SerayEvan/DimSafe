// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::prelude::*;

pub fn local_storage_signal<T>(key: String, default_value: Option<T>) -> Option<RwSignal<T>>
where
    T: Clone + 'static + Send + Sync,
    for<'de> T: serde::Deserialize<'de> + serde::Serialize,
{
    let storage = window()
        .local_storage()
        .expect("could not access localStorage")
        .expect("localStorage not available");

    let value= match storage.get_item(&key) {
        Ok(Some(raw)) => {
            serde_json::from_str(&raw).ok()
        }
        _ => {
            None
        }
    };

    let value= value.or(default_value)?;

    let signal = RwSignal::new(value);

    // Synchronisation -> localStorage
    Effect::new(move |_| {
        let current = signal.get();
        let json = serde_json::to_string(&current).expect("could not serialize signal");
        let _ = storage.set_item(&key, &json);
    });

    return Some(signal)
}

/// Supprime une clé dans `localStorage` et réinitialise le signal.
pub fn destroy_local_storage(key: &str)
{
    if let Ok(Some(storage)) = window().local_storage() {
        let _ = storage.remove_item(key);
    }
}