use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct IpPacket {
    src_ip: String,
    //dst_ip: String,
    data: String,
}
/*
cmd to run the server:
//ngrok http http://localhost:12345
//cargo run --bin external_server
*/
async fn handle_connection(mut stream: TcpStream) {
    println!("Client Connected!");
    //create an array to store the data, data received is about 40 bytes.
    let mut buf = [0u8; 128];
    match stream.read(&mut buf) {
        Ok(n) => {

            let raw_input = String::from_utf8_lossy(&buf[..n]); //turn stream to string
            println!("Raw data received: {}", raw_input);

            match serde_json::from_str::<IpPacket>(&raw_input) { //turn json into struct
                Ok(pkt) => {
                    println!("IP Packet: {:?}", pkt);
                    //Request: "Hello"
                    // -> Response:
                    // "Hi!" (if the IP address is from the USA),
                    // "Bonjour" (if the IP address is from France),
                    // "Hola" (if the IP address is from Spain),
                    // "Reject! Access from the invalid location" (if the IP address is from Byzantium)

                    // Request: "Helllllo"
                    // -> Response: "404 Not Found"

                    let response = match (pkt.src_ip.as_str(), pkt.data.as_str()) {
                        ("1.1.1.1", "Hello") => "Hi!",
                        ("2.2.2.2", "Hello") => "Bonjour",
                        ("3.3.3.3", "Hello") => "Hola",
                        ("4.4.4.4", _) => "Reject! Access from the invalid location",
                        (_, "Helllllo") => "404 Not Found",
                        _ => "Unrecognized request",
                    }; //can further improve. this is simplied version
                    //send response back to client, can package into struct if client adds deserialization
                    if let Err(e) = stream.write_all(response.as_bytes()) {
                        println!("Failed to send response: {}", e);
                    } else {
                        println!("Response sent: {}", response);
                    }
                }
                Err(e) => {
                    println!("Failed to parse JSON: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to read data from client: {}", e);
        }
    }
}
//cmd to run the server:
//cargo run --bin external_server
//ngrok http --url=terribly-amusing-viper.ngrok-free.app 80
fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").expect("Failed to bind this port");
    println!("External server listening on 0.0.0.0:12345");
    for stream in listener.incoming() {
        match stream {
            Ok(tcpstream) => {
                //create a new thread for every client
                thread::spawn(|| handle_connection(tcpstream));
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}