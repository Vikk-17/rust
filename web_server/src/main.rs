use web_server::ThreadPool;
use std::{
    fs,
    thread,
    // time::Duration,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}
};


fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // // println!("The http_request: {http_request:?}");
    // if request_line == "GET / HTTP/1.1" {
    //     let contents = fs::read_to_string("hello.html").unwrap();
    //     let contents_len = contents.len();
    //     let status_line = "HTTP/1.1 200 OK";
    //     let response = format!("Status line: {status_line}\r\nLenght: {contents_len}\r\n\r\nContents: {contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // } else {
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let contents_len = contents.len();
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let response = format!("Status line: {status_line}\r\nLenght: {contents_len}\r\n\r\nContents: {contents}");
    //     stream.write_all(response.as_bytes()).unwrap();
    // }

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT OK", "404.html")
    };


    // simulating latency => blocking the thread by using sleep calls
    // let (status_line, filename) = match &request_line[..] {
    //     "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
    //     "GET /sleep HTTP/1.1" => {
    //         thread::sleep(Duration::from_secs(5));
    //         ("HTTP/1.1 200 OK", "hello.html")
    //     }
    //     _ => ("HTTP/1.1 404 NOT OK", "404.html"),
    // };

    let contents = fs::read_to_string(filename).unwrap();
    let contents_len = contents.len();
    let response = format!("Status line: {status_line}\r\nLenght: {contents_len}\r\n\r\nContents: {contents}");
    stream.write_all(response.as_bytes()).unwrap();

}


fn main() -> std::io::Result<()> {

    // what if I need multiple address
    // let addrs = [
    //     SocketAddr::from(([127, 0, 0, 1], 80)),
    //     SocketAddr::from(([127, 0, 0, 1], 443)),
    // ];

    let listener = TcpListener::bind("127.0.0.1:3000")?;

    // create a pool of 4 threads
    let pool = ThreadPool::new(4);
    // match listener.accept() {
    //     Ok((_socket, addr)) => println!("new client {addr:?}"),
    //     Err(e) => println!("Error {e:?}"),
    // }
    // for stream in listener.incoming() {
    //     // println!("{:?}", stream);
    //     match stream {
    //         Ok(stream) => println!("Ok {:?}", stream),
    //         Err(e) => println!("Error {:?}", e),
    //     }
    // }

    // println!("{:?}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream)=> {
                pool.execute(||{
                    handle_client(stream)
                }) 
            },
            Err(e) => eprintln!("Error: {e:?}"),
        }
    }
    Ok(())
}
