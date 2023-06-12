use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("handling stream: {:?}", stream);
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:4000")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
