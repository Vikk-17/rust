// impl Future<Output = T>, returned by async as a future

use std:: {env, io::Error};
use tokio::net::{TcpListener, TcpStream};
use log::info;
use futures_util::{future, StreamExt, TryStreamExt};

#[tokio::main]
async fn main() -> Result<(), Error> {

    // returns Result<(), SetLoggerError>
    let _ = env_logger::try_init(); // to initialize the global logger with an env logger

    // get the first args after the binary
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // println!("{}", addr);

    // create the event loop and TCP listener, that will accept the connection
    let try_socket = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    // let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);


    // println!("{:?}", &try_socket);

    while let Ok((stream, _)) = try_socket.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    // get the remote peer address
    let addr = stream
        .peer_addr()
        .expect("Connected streams should have a peer address");

    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    // WriteHalf implements Sink & ReadHalf implements Stream
    let (write, read) = ws_stream.split();
    
    // satisfies the api without introducing real asynchronous work
    // try_filter require async predicate
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .forward(write) // takes every item from the stream and sends it into the sink
        .await
        .expect("Failed to forward message");
}
