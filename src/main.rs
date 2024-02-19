mod play;

use play::implement::implement_main;
use std::collections::HashMap;

fn main() -> () {
    let key_arg = match std::env::args().nth(1) {
        Some(k) => k,
        None => panic!("no module key passed"),
    };

    let mut modules: HashMap<String, fn()> = HashMap::new();
    modules.insert("implement".to_string(), implement_main);

    match modules.get(&key_arg) {
        Some(f) => f(),
        None => panic!("invalid module key"),
    };
}
