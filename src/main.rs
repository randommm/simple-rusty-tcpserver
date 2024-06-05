use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:20200").await?;

    loop {
        println!("listening for clients...");
        match listener.accept().await {
            Ok((socket, addr)) => {
                println!("new client: {:?}", addr);
                tokio::spawn(handle_connection(socket));
            }
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap_or_default();
    let addr = stream.peer_addr().unwrap();

    println!(
        "Request: {} from {addr}",
        String::from_utf8_lossy(&buffer[..])
    );

    let response = format!("Hi, {addr}");

    // sleep for 1 second to simulate a slow response
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    stream.write_all(response.as_bytes()).await.unwrap();

    println!("Response: {}", response);
}
