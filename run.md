# How to Compile and Run the Rust Files

## Pulling Project
1. git clone https://github.com/orcaON234/Rust-vpn-project.git
2. cargo update
//split terminal if running both client and server on the same device
## Compile and run server.rs:
//(optional) connect to Web hosting service, for example cloudflare or ngrok
//(optional ngrok http http://localhost:12345 // Set your local host to receive online requests
cargo run --bin external_server

## Compile and run client.rs:
cargo run --bin client

##
ctrl-c to end, it is not terminating by itself and require manual intervention
