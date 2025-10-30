use std::thread;
use std::time::Duration;

fn main() {
    let handles = thread::spawn(move || {
        for i in 1..11 {
            println!("Number {i} from the spawned thread");
            thread::sleep(Duration::from_secs(1));
        }
    });

    handles.join().unwrap();

    for i in 1..6 {
        println!("Number {i} from the main thread");
        thread::sleep(Duration::from_secs(1));
    }

}
