use std::thread;

static X: [i32; 3] = [1, 2, 3];

fn main() {
    thread::spawn(|| dbg!(&X)).join().unwrap();
    thread::spawn(|| dbg!(&X)).join().unwrap();
}
