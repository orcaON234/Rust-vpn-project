# Rust-vpn-project
Team: Very Poisonous Network

Members: Emma(yueqil2), Ocean(oceanng2), Ria(ria6), Kota(kotas3)

# Project description
VPNs are crucial for modern network privacy and a prominent experience into network security. Our goal is to build a functional vpn, since we have suffered from unavailable to access our desired sites overseas as international students. We hope this project can boost our network fundamentals, including packets and protocols, exploring security and encryption and vigilance in real world applications. 


# Technical Overview
Our project is building our customized vpn, implementing a suitable encryption that ensures both performance and safety. The core functions of the VPN we have are : server-client communication, two ways encryption & decryption, packet serialization & deserialization. 

Our own approaches in making our vpn original: 
1. Simulated server - multiple servers
2. Change of protocols for safety (SSL/TCP)
3. Zero knowledge proof
4. IP detection
5.  IP masking

Our project may not complete the list of original approaches, but any of one is sufficient to differentiate our project from other vpn projects. 

Checkpoint 1: (4/10)
We will create the client and server and ensure that there is communication between them.
(Communication should work even between client and server without VPN system)

Checkpoint 2: (4/22)
We will implement encryption and decryption.
This was done using Transport Layer Security (TLS) protocol with the RustLS library.
A configuration was created using a key and certificate for both client and server: certificate is verified (a handshake of sorts) first. Then the current stream is wrapped in TLS from the client end & 

# Possible Challenges
1. Implementation of zero-knowledge proof…
2. Implementation of VPN server
3. Assignment of IP address

# References 
https://www.linkedin.com/pulse/implementing-vpn-server-inrust-luis-soares-m-sc--sqvkf/ 
wireguard-uapi - crates.io: Rust Package Registry 
Implementation of wireguard : gluxon/wireguard-uapi-rs 
Writing a Modern HTTP(S) Tunnel in Rust: https://dzone.com/articles/writing-a-modern-https-tunnel-in-rust
[Secure Your Rust Server! 🔒 Generate Self-Signed SSL & Enable TLS 🦀] https://www.youtube.com/watch?v=PLWq8RE6zSk
