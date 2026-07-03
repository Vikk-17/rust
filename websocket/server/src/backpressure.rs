use std::time::Instant;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};

#[tokio::main()]
async fn main() {
    // channel's capacity is equal to `buffer + num-senders`
    let (tx, mut rx) = mpsc::channel::<usize>(2);

    // slow consumer
    tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("Got: {}", i);
            sleep(Duration::from_secs(2)).await;
        }
    });

    // fast producer
    for i in 0..10 {
        println!("Sending: {}", i);
        let start = Instant::now();
        // sends a value, waiting until there is capacity
        tx.send(i)
            .await
            .unwrap();

        println!("Sent {i} after {:?}", start.elapsed());
    }
}
