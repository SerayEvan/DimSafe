// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use leptos::prelude::*;
use std::sync::Arc;

pub trait DeriveSignal<Derived: Clone + 'static>: Clone + Send + Sync + 'static {
    fn get(&self) -> Derived;
    fn get_untracked(&self) -> Derived;
    fn set(&self, value: Derived) {
        self.update(|v| *v = value);
    }
    fn update<Callback: FnOnce(&mut Derived)>(&self, callback: Callback);
    fn derive<
        C: Clone + Send + Sync + 'static, 
        NewF: for<'b> Fn(&'b mut Derived) -> &'b mut C + 'static
    >(
        self, 
        new_convert_callback: NewF
    ) -> impl DeriveSignal<C> + 'static;
}

pub struct DeriveRwSignal<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> {
    signal: RwSignal<Original>,
    convert_callback: Arc<F>,
}
impl<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> Clone for DeriveRwSignal<Original, Derived, F> {
    fn clone(&self) -> Self {
        Self { signal: self.signal.clone(), convert_callback: self.convert_callback.clone() }
    }
}
unsafe impl<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> Send for DeriveRwSignal<Original, Derived, F> {}
unsafe impl<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> Sync for DeriveRwSignal<Original, Derived, F> {}

impl<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> DeriveRwSignal<Original, Derived, F> {

    pub fn new(signal: &RwSignal<Original>, convert_callback: F) -> Self {
        Self { signal: signal.clone(), convert_callback: Arc::new(convert_callback) }
    }
}

impl<Original: Clone + Send + Sync + 'static> DeriveRwSignal<Original, Original, fn(&mut Original) -> &mut Original> {
    pub fn identity(signal: &RwSignal<Original>) -> Self {
        Self { signal: signal.clone(), convert_callback: Arc::new(|old| old) }
    }
}

impl<Original: Clone + Send + Sync + 'static, Derived: Clone + 'static, F: Fn(&mut Original) -> &mut Derived + 'static> DeriveSignal<Derived> for DeriveRwSignal<Original, Derived, F> {

    fn derive<
        C: Clone + Send + Sync + 'static, 
        NewF: for<'b> Fn(&'b mut Derived) -> &'b mut C + 'static
    >(
        self, 
        new_convert_callback: NewF
    ) -> impl DeriveSignal<C> + 'static
    {
        DeriveRwSignal::new(&self.signal, move |old| {
            let new = (self.convert_callback)(old);
            new_convert_callback(new)
        })
    }
    
    fn update<Callback: FnOnce(&mut Derived)>(&self, callback: Callback) {
        self.signal.update(move |old| {
            let new = (self.convert_callback)(old);
            callback(new);
        });
    }

    fn get(&self) -> Derived {
        let mut a = self.signal.get();
        let b = (self.convert_callback)(&mut a);
        b.clone()
    }

    fn get_untracked(&self) -> Derived {
        let mut a = self.signal.get_untracked();
        let b = (self.convert_callback)(&mut a);
        b.clone()
    }
}