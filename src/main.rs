// main.rs
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    let backends = Arc::new(Mutex::new(HashMap::new()));
    let backends_clone = backends.clone();

    // Ajouter les server 1 et 2
    {
        let mut backends = backends_clone.lock().unwrap();
        backends.insert("server1".to_string(), "127.0.0.1:8081".parse().unwrap());
        backends.insert("server2".to_string(), "127.0.0.1:8082".parse().unwrap());
    }

    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Load balancer démarré sur 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let backends = backends_clone.clone();
        tokio::spawn(async move {
            process(socket, backends).await;
        });
    }
}

async fn process(mut socket: TcpStream, backends: Arc<Mutex<HashMap<String, SocketAddr>>>) {
    let mut buffer = [0; 1024];
    match socket.read(&mut buffer).await {
        Ok(_) => {
            let request = std::str::from_utf8(&buffer).unwrap();
            println!("Requête reçue: {}", request.trim_end_matches(char::from(0)));

            let backend_name = get_backend_name(request);
            let backend_addr = backends.lock().unwrap().get(&backend_name).unwrap().clone();           
            let mut backend_socket = TcpStream::connect(backend_addr).await.unwrap();
            backend_socket.write_all(&buffer).await.unwrap();

            let mut backend_buffer = [0; 1024];
            let n = backend_socket.read(&mut backend_buffer).await.unwrap();
            socket.write_all(&backend_buffer[..n]).await.unwrap();
        }
        Err(e) => {
            println!("Erreur socket: {}", e);
        }
    }
}

fn get_backend_name(_request: &str) -> String {
    // Logique pour sélectionner le backend en fonction de la requête
    // Ici, on utilise un round-robin simple
    static COUNTER: Mutex<usize> = Mutex::new(0);
    let mut counter = COUNTER.lock().unwrap();
    let name = if *counter == 0 {
        *counter = 1;
        "server1".to_string()
    } else {
        *counter = 0;
        "server2".to_string()
    };
    name
}
