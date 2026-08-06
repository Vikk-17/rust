mod models;

use models::DnsHeader;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
        println!("Listening on: (127.0.0.1:2053)");

        println!();
        let socket = UdpSocket::bind("127.0.0.1:2053")?;

    loop {
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        println!("[+] Source Address: {}:{}\n[+] Number of bytest recieved: {amt}",
            src.ip(),
            src.port()
        );

        let buf = &mut buf[..amt];
        buf.reverse();
        println!("[+] Raw bytes: {:?}", buf);
        // socket.send_to(buf, &src)?;

        println!();
    }
}
