mod play;

use std::collections::HashMap;

fn main() -> () {
    let key_arg = match std::env::args().nth(1) {
        Some(k) => k,
        None => panic!("no module key passed"),
    };

    let mut modules: HashMap<String, fn()> = HashMap::new();
    modules.insert("impl".to_string(), play::impl_keyword::impl_keyword_main);
    modules.insert("trait".to_string(), play::trait_keyword::trait_keyword_main);

    match modules.get(&key_arg) {
        Some(f) => f(),
        None => panic!("invalid module key"),
    };
}
