use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::thread;

fn main() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // spawn other thread
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            println!("Test");
        }
    });

    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: help, stop"),
            "stop" => break,
            cmd => println!("unknown commands: {cmd:?}"),
        }
    }

    STOP.store(true, Relaxed);
    background_thread.join().unwrap();
}
