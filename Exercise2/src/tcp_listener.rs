use std::io::{Read, Write}; // Gir read()/write_all() til TcpStream
use std::net::TcpListener; // TCP listener (server-side socket)
use std::thread; // For å starte nye tråder
use std::thread::sleep; // sleep()-funksjonen
use std::time::Duration; // Tidsintervall for sleep

fn main() {
    // Porten vi vil lytte på for callback-forbindelser fra serveren
    let my_port = 40000;

    // Oppretter en TCP-listener som lytter på alle nettverkskort
    let listener = TcpListener::bind(format!("0.0.0.0:{}", my_port)).unwrap();
    println!("Listening on port {}", my_port);

    // incoming() er en blokkerende accept-loop
    // Den venter til en ny TCP-forbindelse kommer inn
    for stream in listener.incoming() {
        // unwrap Result<TcpStream, Error> -> TcpStream
        let mut stream = stream.unwrap();

        // Starter en ny tråd for å håndtere denne forbindelsen
        thread::spawn(move || {
            // Buffer for mottatte bytes
            let mut buf = [0u8; 1024];

            loop {
                // Leser data fra TCP-forbindelsen
                let n = stream.read(&mut buf).unwrap();

                // n == 0 betyr at motparten har lukket forbindelsen
                if n == 0 {
                    break;
                }

                // Tolker mottatte bytes som UTF-8 tekst
                let msg = std::str::from_utf8(&buf[..n]).unwrap();
                println!("Callback received: {}", msg);

                // Ekkoer nøyaktig de mottatte bytene tilbake
                stream.write_all(&buf[..n]).unwrap();

                // Pause for å unngå spam / tydeligere logging
                sleep(Duration::from_millis(1000));
            }
        });

        // Valgfri pause mellom nye innkommende forbindelser
        // (ikke strengt nødvendig, men ufarlig)
        sleep(Duration::from_millis(1000));
    }
}
