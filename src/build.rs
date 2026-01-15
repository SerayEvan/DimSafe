// SPDX-License-Identifier: Apache-2.0
// Copyright 2025 Evan SERAY

use lalrpop;
use lalrpop::Configuration;

fn main() {
    Configuration::new()
        .generate_in_source_tree()
        .process()
        .unwrap();
    //lalrpop::process_root().unwrap();
}