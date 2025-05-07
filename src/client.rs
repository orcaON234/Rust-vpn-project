use std::io::{self, Write, BufReader, Read};
use std::net::TcpStream;
use std::sync::Arc;
use serde::Serialize;
use rustls::{StreamOwned, ServerName};

use rustls::{server, Certificate, ClientConfig, ClientConnection, PrivateKey, Stream};
use rustls::internal::pemfile::{certs, rsa_private_keys};


fn encryptions() -> ClientConfig {
    //STEP 1: Take the server's certificate, retrieve the actual certificate from the file
    //STEP 2: Create and return a Client Configuration using this certificate
    let certificate_file = String::from("Some certificate from the server"); //placeholder for actual cert file
    //WHEN WE FIND THE FILE AND ITS FORMAT, use File lib to read: = File::open(file_name);

    //certs requires BufReader
    let cert_read = BufReader::new(certificate_file);
    let actual_certificates = certs(&mut cert_read).expect("Couldn't read certificate.")[0];
    if (actual_certificates.is_empty()) {
        println!("No certificate found...");
        return;
    }
    let mut configuration = ClientConfig::new();
    configuration.root_store.add(&actual_certificates[0]).expect("Failed.");
    //root store: contains trusted certificates.
    return configuration;
}

//need to work on error handling and exception throwing in the encrypted parts.


#[derive(Serialize)]
struct IpPacket {
    src_ip: String,
    data: String,
}

// As simplified vpn soft
fn select_vpn_ip() -> String {
    println!("Choose a country:");
    println!("1. USA\n2. France\n3. Spain\n4. Byzantium");

    print!("Enter country name: ");
    io::stdout().flush().unwrap();
    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();
    match country.trim().to_lowercase().as_str() {
        "usa" => "1.1.1.1".to_string(),
        "france" => "2.2.2.2".to_string(),
        "spain" => "3.3.3.3".to_string(),
        "byzantium" => "4.4.4.4".to_string(),
        _ => {
            eprintln!("Invalid country selection.");
            std::process::exit(1);
        }
    }
}


fn main() {
    let port = "127.0.0.1:12345"; // connect to communication function
    let client_config = Arc::new(encryptions());

    match TcpStream::connect(port) {
        Ok(stream) => {
            println!("Connected to external server at {}", port);
            
            //Wrapping TCPStream with configured TLS:
            //first configure official name for server, feed the created configuration and server name and modify the current stream
            let server_name = ServerName::try_from("Host").expect("wrong server");
            let connection = ClientConnection::new(client_config, server_name)
                .expect("Failed to create TLS connection");
            let mut tls_encrypted_stream = StreamOwned::new(connection, stream);

            // choose vpn mode or not
            print!("Use VPN mode? (yes / no): ");
            io::stdout().flush().unwrap();
            let mut mode = String::new();
            io::stdin().read_line(&mut mode).unwrap();
            let mode = mode.trim().to_lowercase();

            let src_ip = match mode.as_str() {
                "yes" => select_vpn_ip(),
                "no" => "1.1.1.1".to_string(), // default source ip
                _ => {
                    eprintln!("Invalid input: please enter 'yes' or 'no'.");
                    std::process::exit(1);
                }
            };

            // data input by user
            print!("Enter request message (e.g., Hello): ");
            io::stdout().flush().unwrap();
            let mut data = String::new();
            io::stdin().read_line(&mut data).unwrap();
            let data = data.trim().to_string();

            // make ip packet to send from the data user gave
            let packet = IpPacket {
                src_ip,
                data,
            };
            let json_packet = serde_json::to_string(&packet).expect("Serialization failed");

            // send ip packet to external server
            if let Err(e) = tls_encrypted_stream.write_all(json_packet.as_bytes()) {
                eprintln!("[x] Failed to send packet: {}", e);
                return;
            } else {
                println!("[<] Packet sent successfully: {}", json_packet);
            }
            

            // read response
            let mut buf = [0u8; 128];
            match tls_encrypted_stream.read(&mut buf) {
                Ok(n) => {
                    let response = String::from_utf8_lossy(&buf[..n]);
                    println!("[<] Server response: {}", response);
                }
                Err(e) => {
                    eprintln!("[x] Failed to read response: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("[x] Failed to connect to server: {}", e);
        }
    }
}

