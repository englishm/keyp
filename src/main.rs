use std::{
    collections::HashMap,
    error,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn handle_client(
    stream: TcpStream,
    mut kv_store: HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    // TODO split this all apart to make parsing testable

    // allocate 1 MB buffer to hold request data
    let mut buf = [0; 1_000_000];
    println!("handling stream: {:?}", stream);

    // read at most 1MB of data into buffer
    let mut handle = stream.try_clone()?.take(1_000_000);
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
            eprintln!("Something went wrong");
            respond_not_ok(stream, "")?;
            return Ok(kv_store);
        }
    }

    // TODO: handle more registered methods:
    // https://www.iana.org/assignments/http-methods/http-methods.xhtml

    // HTTP/1.1 Request lines:
    // https://www.rfc-editor.org/rfc/rfc9112#section-3
    // request-line   = method SP request-target SP HTTP-version
    // TODO: reject too long methods and request-target URIs
    let request_target: &str;
    match request_line.split(' ').collect::<Vec<_>>()[..] {
        [method, rt, http_version] => {
            request_target = rt;
            println!("{} request", method);
            println!("request-target: {}", request_target);
            println!("HTTP-version: {}", http_version);
        }
        _ => {
            eprintln!("Failed to parse request line: {:?}", request_line);
            todo!("🤷");
        }
    }

    // request-target:
    // https://www.rfc-editor.org/rfc/rfc9112#section-3.2
    //   request-target = origin-form
    //                  / absolute-form
    //                  / authority-form
    //                  / asterisk-form
    // TODO: properly handle anything other than origin form

    // When your server receives a request on `http://localhost:4000/set?somekey=somevalue`
    // it should store the passed key and value in memory.
    if request_target.starts_with("/set?") {
        println!("/set?");
        // TODO parse parameters of request URI
        let parameters;
        match request_target.split('?').collect::<Vec<_>>()[..] {
            [_, params] => {
                parameters = params;
                println!("parameters: {}", parameters)
            }
            _ => {
                eprintln!("Something went wrong");
                respond_not_ok(stream, "")?;
                return Ok(kv_store);
            }
        }
        match parameters.split('=').collect::<Vec<_>>()[..] {
            [k, v] => {
                println!("k: {}", k);
                println!("v: {}", v);
                println!("Setting {}={}", k, v);
                kv_store.insert(k.to_string(), v.to_string());
                respond_ok(stream, "")?;
            }
            _ => {
                eprintln!("Something went wrong");
                respond_not_ok(stream, "")?;
                return Ok(kv_store);
            }
        }
    }
    // When it receives a request on `http://localhost:4000/get?key=somekey`
    // it should return the value stored at `somekey`.
    else if request_target.starts_with("/get?") {
        println!("/get?");
        let parameters;
        match request_target.split('?').collect::<Vec<_>>()[..] {
            [_, params] => {
                parameters = params;
                println!("parameters: {}", parameters)
            }
            _ => {
                eprintln!("Something went wrong");
                respond_not_ok(stream, "")?;
                return Ok(kv_store);
            }
        }
        if parameters.contains('=') {
            eprintln!("Something went wrong");
            respond_not_ok(stream, "")?;
            return Ok(kv_store);
        }
        let k = parameters;
        let mut value = &String::new();
        match kv_store.get(k) {
            Some(v) => {
                value = v;
            }
            None => {
                eprintln!("No value for key: {}", k);
            }
        }
        respond_ok(stream, value)?;
    } else {
        eprintln!("Neither setting nor getting?");
        respond_not_ok(stream, "")?;
    }

    Ok(kv_store)
}

fn respond_ok(mut stream: TcpStream, body: &str) -> Result<()> {
    let mut response: Vec<u8> = Vec::new();
    response.extend("HTTP/1.1 200 OK\n".as_bytes().to_vec());
    // TODO: think about content types
    response.extend("\n".as_bytes().to_vec());
    response.extend(body.as_bytes().to_vec());
    stream.write_all(response.as_slice())?;
    stream.flush()?;
    Ok(())
}

// Assume if anything goes wrong, it's our fault
fn respond_not_ok(mut stream: TcpStream, body: &str) -> Result<()> {
    let mut response: Vec<u8> = Vec::new();
    response.extend("HTTP/1.1 500 Internal Server Error\n".as_bytes().to_vec());
    response.extend("\n".as_bytes().to_vec());
    response.extend(body.as_bytes().to_vec());
    stream.write_all(response.as_slice())?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<()> {
    println!("Hello, world!");
    // write a program that runs a server that is accessible on `http://localhost:4000/`
    let listener = TcpListener::bind("127.0.0.1:4000")?;

    let mut kv_store = HashMap::new();

    let mut f = File::open("keyp.db")?;

    // first read from the file
    // format:
    // <key> <value>
    //

    let mut db_str = String::new();
    let _bytes = f.read_to_string(&mut db_str)?;

    for line in db_str.lines() {
        match line.split(' ').collect::<Vec<&str>>()[..] {
            [key, value] => {
                dbg!(key, value);
                kv_store.insert(key.into(), value.into());
            }
            _ => {
                dbg!(line);
                panic!()
            }
        }
    }

    // Not async or even multi-threaded (yet), but this
    // is definitely a feature for now because we can
    // easily avoid data races in our toy database XD
    for stream in listener.incoming() {
        // let timestamp = format!("{:?}", SystemTime::now());
        // kv_store.insert("request_time".to_string(), timestamp);
        println!("initial kv: {:?}", kv_store);
        kv_store = handle_client(stream?, kv_store)?;
        println!("resulting kv: {:?}", kv_store);
    }

    Ok(())
}
