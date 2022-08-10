use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);

        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer: [u8; 1024]  = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status, filename) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep)  {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
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
