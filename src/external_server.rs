use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, BufReader};
use std::thread;
use serde::Deserialize;
use rustls::{server, ServerConfig};
use rustls::internal::pemfile::{certs, rsa_private_keys};

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
                        _ => "404 Not Found",
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


fn for_encryption() -> ServerConfig {
    //need to add expect statement for when certificate/key can't be opened.
    let certificate_file = String::from("some_certificate"); //need to find a certificate file to be able to test this on
    let key_file: String = String::from("some_key"); //the files are just placeholders.
    //when file found: File::open(file_name);

    let mut cert_read = BufReader::new(certificate_file);;
    let mut key_read = BufReader::new(key_file);

    let mut encryption = ServerConfig::new(rustls::NoClientAuth::new()); //client authentication not required.
    let actual_certificate = certs(&mut cert_read).expect("Failed to read certificate");
    let actual_key = rsa_private_keys(&mut key_read)[0].clone().expect("Failed to read key");
    if (actual_key.is_empty() || actual_certificate.is_empty()) {
        println!("Error: certificate or key is empty");
        return;
    }
    encryption.set_single_cert(actual_certificate, actual_key).expect("Failed");
    return encryption;
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:12345").expect("Failed to bind this port");
    let this_encryption = Arc::new(for_encryption());
    println!("External server listening on 0.0.0.0:12345");
    for stream in listener.incoming() {
        match stream {
            Ok(tcpstream) => {
                //create a new thread for every client
                thread::spawn(move || {
                    let mut serverconnection = rustls::ServerConnection::new(Arc::clone(&this_encryption)).expect("Failed to connect");
                    let mut tls_stream = StreamOwned::new(serverconnection, tcpstream);
                    handle_connection(tcpstream);
            });
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e);
            }
        }
    }
}


