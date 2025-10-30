use std::thread;

fn main() -> std::thread::Result<()> {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("This is main thread");

    t1.join().unwrap();
    t2.join().unwrap();

    // let mut threads = Vec::new();
    let numbers = vec![1, 2, 3];

    thread::spawn(move || {
        // f();
        for n in numbers {
            println!("{n}");
        }
    }).join().unwrap();

    // for handle in threads {
    //     handle.join().unwrap();
    // }

    Ok(())
}


fn f() {
    println!("Hello from another thread!");
    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
