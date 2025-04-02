use std::io::{self, Write};
use std::net::TcpStream;

fn main() {
    match TcpStream::connect("10.191.17.64:12345") {
        Ok(mut stream) => {
            println!("Connected to external server at 10.191.17.64:12345");
            print!("Enter payload to send: ");
            io::stdout().flush().unwrap();

            let mut payload = String::new();
            io::stdin().read_line(&mut payload).unwrap();

            if let Err(e) = stream.write_all(payload.as_bytes()) {
                println!("[x] Failed to send payload: {}", e);
            } else {
                println!("[<] Payload sent successfully.");
            }
        }
        Err(e) => {
            println!("[x] Failed to connect to server: {}", e);
        }
    }
}