use std::thread;
use std::sync::mpsc;
// use std::time::Duration;
// use std::{thread, time::Duration};

// fn main() {
//     // thread::spawn( || {
//     //     for i in 1..100 {
//     //         println!("Number from spawn thread {i}");
//     //         thread::sleep(Duration::from_millis(1));
//     //     }
//     // });
//     //
//     // for i in 1..100 {
//     //     println!("Number from main thread {i}");
//     //     thread::sleep(Duration::from_millis(1));
//     //
//     // }
//     let v:Vec<i32> = vec![1, 2, 3];
//     let handle = thread::spawn(move || {
//         println!("Here is the vector: {:?}", v);
//     });
//     handle.join().unwrap();
// }

// message passing

// fn main() {
//    let (tx, rx) = mpsc::channel();
//    thread::spawn(move || {
//        let value = String::from("Hofstader");
//        tx.send(value).unwrap();
//    });
//
//    let recieved = rx.recv().unwrap();
//    println!("Recieved value {recieved}");
// }

fn main(){
    let (tx, rx) = mpsc::channel();
    for i in 0..10 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }
    drop(tx);

    let mut ans:u64 = 0;
    for val in rx {
        ans = ans + val;
    }
    println!("Answer is: {ans}");
}
