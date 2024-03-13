/* https://doc.rust-lang.org/std/thread/index.html */

use std::{thread, time::Duration};

pub fn thread_std_main() -> () {
    let mut power: u8 = 0;

    /* store a reference to the thread otherwise it's 'detached' with no way to know it's done */
    let thread_handle = thread::spawn(move || -> u8 {
        for _ in 0..5 {
            println!("thread one running -> {}", power);
            thread::sleep(Duration::from_millis(100));
            power += 10;
        }

        power
    });

    /* wait for the thread to return */
    let thread_handle_result = thread_handle.join();
    match thread_handle_result {
        Ok(res) => println!("power reading after thread finished -> {}", res),
        Err(err) => println!("uh oh -> {:?}", err),
    }
}
