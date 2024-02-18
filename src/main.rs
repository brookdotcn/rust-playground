use std::collections::HashMap;

fn example_module() -> () {
    println!("Hello, world!");
}

fn main() -> () {
    let key_arg = match std::env::args().nth(1) {
        Some(k) => k,
        None => panic!("no module key passed"),
    };

    let mut modules: HashMap<String, fn()> = HashMap::new();
    modules.insert("example".to_string(), example_module);

    match modules.get(&key_arg) {
        Some(f) => f(),
        None => panic!("invalid module key"),
    };
}
