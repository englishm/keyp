use std::{
    error,
    io::Read,
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn handle_client(stream: TcpStream) -> Result<()> {
    // allocate 1 MB buffer to hold request data
    let mut buf = [0; 1_000_000];
    println!("handling stream: {:?}", stream);

    // read at most 1MB of data into buffer
    let mut handle = stream.take(1_000_000);
    handle.read(&mut buf)?;

    // print request as a UTF-8 string
    let s = String::from_utf8(buf.to_vec())?;
    println!("s: {}", s); // print request

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
