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
    println!("{}", s); // print request

    // TODO: HTTP Parsing
    // Can we hack together minimal parsing with nothing but std?
    // Or would it be OK to pull in a minmal lib here like httpparse?
    // Let's see how far we can get with something hacky
    //
    // TODO: HTTP/2 and HTTP/3 support
    // This is where we'd probably want to switch over to a library

    // DONE: Ensure we have a complete request before preceding
    // On second thought, we don't actually need that to fulfill
    // the requirements as written..

    // Note: regex isn't even in std so this could get interesting..
    // Good news is we have slice pattern matching...
    let mut lines = s.lines();
    let request_line;
    match lines.next() {
        Some(value) => {
            request_line = value;
        }
        None => {
            request_line = "";
            eprintln!("Something went wrong")
            // TODO: Return 500 error
        }
    }

    // TODO: handle more registered methods:
    // https://www.iana.org/assignments/http-methods/http-methods.xhtml

    // HTTP/1.1 Request lines:
    // https://www.rfc-editor.org/rfc/rfc9112#section-3
    // request-line   = method SP request-target SP HTTP-version
    match request_line.split(' ').collect::<Vec<_>>()[..] {
        ["GET", request_target, http_version] => {
            println!("GET request");
            println!("request-target: {}", request_target);
            println!("HTTP-version: {}", http_version);
        }
        ["PUT", request_target, http_version] => {
            println!("PUT request");
            println!("request-target: {}", request_target);
            println!("HTTP-version: {}", http_version);
        }
        // ["HEAD", request_target, http_version] => {
        //     println!("HEAD request");
        //     println!("request-target: {}", request_target);
        //     println!("HTTP-version: {}", http_version);
        // }
        _ => {
            eprintln!("Failed to parse request line: {:?}", request_line);
            todo!("🤷");
        }
    }

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
