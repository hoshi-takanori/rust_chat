use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

// todo 1
// define a struct or class to hold the connection and user name
// problem: how to define a struct or class and understand what I can or cannot do with them

// todo 2
// define a global variable (?) to hold the array or vector of above thing
// problem: how to define a global variable, is it possible?

// todo 3
// add or remove an item to or from the array when connected or disconnected
// problem: how to share the global variable between threads

// todo 4
// define a function to broadcast to all connections
// problem: how to communicate to other threads

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
