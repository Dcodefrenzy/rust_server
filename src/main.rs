use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024]  = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
    }else {
        ("HTTP/1.1 404 OK", "404.html")
    };

    let contents =  fs::read_to_string(filename).unwrap();

    //println!("Request {}", String::from_utf8_lossy(&buffer[..]));

    let response = format!(
        "{}\r\nContent-Length: {}r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}
