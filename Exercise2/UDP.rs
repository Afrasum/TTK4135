use std::net::{IpAddr, SocketAddr, UdpSocket};

fn main() {
    let addr: SocketAddr = "0.0.0.0:30000".parse().unwrap();
    let recieve_socket: UdpSocket = UdpSocket::bind(addr).unwrap();

    let mut buffer = [0u8; 1024];

    let local_ip: IpAddr = "192.168.1.50".parse().unwrap();

    loop {
        // numBytesReceived = recvSock.receiveFrom(buffer, ref fromWho)
        let (num_bytes_received, from_who) = recieve_socket.recv_from(&mut buffer).unwrap();

        // if from_who.ip() != local_ip {
        let msg = std::str::from_utf8(&buffer[..num_bytes_received]).unwrap();
        println!("Received: {}", msg);
        // }
    }
}
