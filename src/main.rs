mod toys;

use std::collections::HashMap;

fn main() -> () {
    let key_arg = match std::env::args().nth(1) {
        Some(k) => k,
        None => panic!("no module key passed"),
    };

    let mut modules: HashMap<String, fn()> = HashMap::new();
    modules.insert("impl".to_string(), toys::impl_keyword::impl_keyword_main);
    modules.insert("trait".to_string(), toys::trait_keyword::trait_keyword_main);
    modules.insert("arc".to_string(), toys::arc_struct::arc_struct_main);
    modules.insert("thread".to_string(), toys::thread_std::thread_std_main);

    match modules.get(&key_arg) {
        Some(f) => f(),
        None => panic!("invalid module key"),
    };
}
