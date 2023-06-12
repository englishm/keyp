use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("handling stream: {:?}", stream);
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    // Not async or even multi-threaded (yet), but maybe this
    // is actually a feature for now because we can avoid
    // data races in our toy database XD
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
