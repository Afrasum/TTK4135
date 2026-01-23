use std::io::{Read, Write}; // Gir read()/write_all() til TcpStream
use std::net::{TcpListener, TcpStream}; // TCP server + client
use std::thread; // For tråder
use std::time::Duration; // For sleep

fn main() {
    // ===== KONFIGURASJON =====

    let server_ip = "127.0.0.1"; // IP til TCP-serveren
    let server_port = 33546; // Port for \0-terminert TCP
    let my_ip = "127.0.0.1"; // Din IP (må være nåbar av serveren)
    let my_port = 40000; // Port du vil lytte på

    // ===== START LYTTER (SERVER-ROLLE) =====

    // Lager en TCP-listener som lytter på alle interfaces på my_port
    let listener = TcpListener::bind(format!("0.0.0.0:{}", my_port)).unwrap();

    // Starter en tråd som aksepterer innkommende forbindelser
    thread::spawn(move || {
        // Loop over alle innkommende TCP-forbindelser
        for stream in listener.incoming() {
            let mut stream = stream.unwrap(); // Faktisk TCP-stream

            // Egen tråd per forbindelse
            thread::spawn(move || {
                let mut buf = [0u8; 1024]; // Buffer for mottak

                loop {
                    // Leser bytes fra forbindelsen
                    let n = stream.read(&mut buf).unwrap();

                    // Hvis n == 0, er forbindelsen lukket
                    if n == 0 {
                        break;
                    }

                    // Tolker mottatte bytes som UTF-8 tekst
                    let msg = std::str::from_utf8(&buf[..n]).unwrap();

                    // Printer meldingen
                    println!("Callback received: {}", msg);

                    // Ekko meldingen tilbake
                    stream.write_all(&buf[..n]).unwrap();
                }
            });
        }
    });

    // ===== KLIENT-ROLLE MOT HOVEDSERVER =====

    // Kobler til TCP-serveren
    let mut stream = TcpStream::connect(format!("{}:{}", server_ip, server_port)).unwrap();

    // Setter TCP_NODELAY for å unngå sammenslåing av pakker
    stream.set_nodelay(true).unwrap();

    // Lager "Connect to"-meldingen
    let connect_msg = format!("Connect to: {}:{}\0", my_ip, my_port);

    // Sender meldingen til serveren
    stream.write_all(connect_msg.as_bytes()).unwrap();

    // Buffer for mottak fra serveren
    let mut buf = [0u8; 1024];

    // ===== SEND / MOTTA ECHO =====

    let mut counter = 0;

    loop {
        // Lager melding som slutter med \0
        let msg = format!("Hello {}\0", counter);

        // Sender melding til serveren
        stream.write_all(msg.as_bytes()).unwrap();

        // Leser svar fra serveren
        let n = stream.read(&mut buf).unwrap();

        // Tolker svaret
        let reply = std::str::from_utf8(&buf[..n]).unwrap();

        // Printer svaret
        println!("Server replied: {}", reply);

        // Teller opp
        counter += 1;

        // Sover litt for å være snill mot serveren
        thread::sleep(Duration::from_millis(1000));
    }
}
