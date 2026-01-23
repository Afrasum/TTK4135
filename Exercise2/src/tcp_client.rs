use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // === KONFIG ===
    let server_addr = "127.0.0.1:33546"; // TCP-server (\0-terminert)
    let my_ip = "127.0.0.1";
    let my_port = 40000;

    // === CONNECT TIL SERVER ===
    let mut stream = TcpStream::connect(server_addr).unwrap();
    stream.set_nodelay(true).unwrap();

    // === BE SERVEREN KOBLE TILBAKE ===
    let connect_msg = format!("Connect to: {}:{}\0", my_ip, my_port);
    stream.write_all(connect_msg.as_bytes()).unwrap();

    let mut buf = [0u8; 1024];
    let mut counter = 0;

    // === SEND / MOTTA ECHO ===
    loop {
        let msg = format!("Hello {}\0", counter);
        stream.write_all(msg.as_bytes()).unwrap();

        let n = stream.read(&mut buf).unwrap();
        let reply = std::str::from_utf8(&buf[..n]).unwrap();
        println!("Server replied: {}", reply);

        counter += 1;
        sleep(Duration::from_millis(1000));
    }
}
