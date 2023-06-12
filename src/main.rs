use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    println!("handling stream: {:?}", stream);
    // When your server receives a request on `http://localhost:4000/set?somekey=somevalue`
    // it should store the passed key and value in memory.

    // When it receives a request on `http://localhost:4000/get?key=somekey`
    // it should return the value stored at `somekey`.
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    // write a program that runs a server that is accessible on `http://localhost:4000/`
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    // Not async or even multi-threaded (yet), but maybe this
    // is actually a feature for now because we can avoid
    // data races in our toy database XD
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}
