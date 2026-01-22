use std::net::{IpAddr, SocketAddr, UdpSocket};

fn main() {
    let addr: SocketAddr = "0.0.0.0:30000".parse().unwrap();
    let recieve_socket: UdpSocket = UdpSocket::bind(addr).unwrap();

    let mut buffer = [0u8; 1024];

    let local_ip: IpAddr = "192.168.1.50".parse().unwrap();

    let server_addr: SocketAddr = "127.0.0.1:20000".parse().unwrap();

    let sock: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();

    sock.connect(server_addr).unwrap();
    let mut i = 0;

    loop {
        // numBytesReceived = recvSock.receiveFrom(buffer, ref fromWho)
        let (num_bytes_received, from_who) = recieve_socket.recv_from(&mut buffer).unwrap();

        // if from_who.ip() != local_ip {
        let msg = std::str::from_utf8(&buffer[..num_bytes_received]).unwrap();
        println!("Received: {}", msg);
        // }

        let msg_to_send = format!("Hello {}", i);

        sock.send(msg_to_send.as_bytes()).unwrap();
        i += 1;
    }
}
