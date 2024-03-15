mod game;

use std::collections::HashMap;

fn main() -> () {
    let module_input = std::env::args().nth(1).expect("no module passed");

    let mut modules = HashMap::<String, fn()>::new();
    modules.insert("hangman".to_string(), game::hangman::hangman_run);

    match modules.get(&module_input) {
        Some(f) => f(),
        None => panic!("unknown module key"),
    }
}
