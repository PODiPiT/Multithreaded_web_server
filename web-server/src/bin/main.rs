use std::{fs, thread};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use web_server::ThreadPool;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.01:7878").unwrap(); //this line allow us to listen TCP connections at IP address

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) { // incoming returns an iterator over the connections being received by listener
        let stream = stream.unwrap(); // we use shadowing to extract TcpStream

        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection (mut stream: TcpStream) { // reads data from tcp connection and print it out
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let(status_line, filename) =
        if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK", "index.html")
        } else if buffer.starts_with(sleep){
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length : {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}