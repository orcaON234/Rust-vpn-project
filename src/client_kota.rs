use std::net::TcpStream;
use std::io::{Read, Write};
use serde::Serialize;

#[derive(Serialize)]
struct IpPacket {
    src_ip: String,  
    //dst_ip: String,
    data: String,  
}

fn main() {
    let server_addr = "127.0.0.1:12345"; // The address of the external server

    // Create an example packet to send
    let packet = IpPacket {
        src_ip: "1.1.1.1".to_string(),  // Simulate a request from USA
        //dst_ip: <- For now, we use TcpStream instead of dst_ip
        data: "Hello".to_string(),      // Request message
    };

    // Serialize the packet to JSON format
    let json_packet = serde_json::to_string(&packet).expect("Failed to serialize packet");

    // Try to connect to the server
    match TcpStream::connect(server_addr) {
        Ok(mut stream) => {
            // Send the JSON packet to the server
            stream.write_all(json_packet.as_bytes()).expect("Failed to send packet");

            // Prepare a buffer to read the server's response
            let mut buffer = [0u8; 128];
            match stream.read(&mut buffer) {
                Ok(n) => {
                    // Convert response bytes into a UTF-8 string and print it
                    let response = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received response: {}", response);
                }
                Err(e) => {
                    println!("Failed to read response: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect to server: {}", e);
        }
    }
}


/*  what Ria implemented
fn main() {
    let port = "0.0.0.0:12345"; // Port to listen on
    let client_config = Arc::new(encryptions());
    match TcpStream::connect(port) {
        Ok(mut stream) => {
            println!("Connected to external server at {}", port);


            //Wrapping TCPStream with configured TLS:
            //first configure official name for server, feed the created configuration and server name and modify the current stream
            let server_name = ServerName::try_from("Host").expect("wrong server");
            let connection = ClientConnection::new(client_config, server_name);
            let mut tls_encrypted_stream = StreamOwned::new(connection, stream);


            print!("Enter payload to send: ");
            io::stdout().flush().unwrap();

            let mut payload = String::new();
            io::stdin().read_line(&mut payload).unwrap();

            if let Err(e) = tls_encrypted_stream.write_all(payload.as_bytes()) {
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
*/