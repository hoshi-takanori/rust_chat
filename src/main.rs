use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("localhost:9000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    print!("message: {}", String::from_utf8_lossy(&buffer[..]));

    stream.write(&buffer).unwrap();
    stream.flush().unwrap();
}
