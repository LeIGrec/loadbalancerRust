// server2.rs (serveur backend 2)
use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();
    println!("Serveur 2 démarré sur 127.0.0.1:8082");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        let request = std::str::from_utf8(&buffer).unwrap().trim_end_matches(char::from(0));
        println!("Requête reçue sur le serveur 2: {}", request);

        let response = "HTTP/1.1 200 OK\r\nContent-Length: 21\r\n\r\nHello from Server 2!\n".as_bytes();
        stream.write_all(response).unwrap();
    }
}
