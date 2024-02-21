/* https://doc.rust-lang.org/std/sync/struct.Arc.html */

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use std::sync::Arc;
use std::{thread, time::Duration};

struct EngineerStatus {
    lines_of_code_written: AtomicUsize,
}

pub fn arc_struct_main() -> () {
    let engineer = Arc::new(EngineerStatus {
        lines_of_code_written: AtomicUsize::new(0),
    });
    let shared_engineer = engineer.clone();

    thread::spawn(move || {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(250));
            engineer.lines_of_code_written.fetch_add(1, Relaxed);
            println!("previous -> {} | after -> {}", i, i + 1);
        }
    });

    while shared_engineer.lines_of_code_written.load(Relaxed) < 10 {
        println!("waiting...");
        thread::sleep(Duration::from_secs(1));
    }
}
