use std::io::{self, Write, BufReader};
use std::net::TcpStream;
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
