use std::{
    error,
    io::Read,
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    let mut buf: Vec<u8> = Vec::new();
    println!("handling stream: {:?}", stream);
    stream.read_to_end(&mut buf)?;
    println!("buf: {:?}", buf);
    // When your server receives a request on `http://localhost:4000/set?somekey=somevalue`
    // it should store the passed key and value in memory.

    // When it receives a request on `http://localhost:4000/get?key=somekey`
    // it should return the value stored at `somekey`.
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    // write a program that runs a server that is accessible on `http://localhost:4000/`
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    // Not async or even multi-threaded (yet), but maybe this
    // is actually a feature for now because we can avoid
    // data races in our toy database XD
    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
