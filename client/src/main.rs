use std::net::TcpStream;
use std::io::Write;

use std::error::Error as StdError;
use std::io::stdin;
use std::result::Result as StdResult;

use std::{thread, time};
use std::sync::mpsc::channel;



fn main() {

    let (tx, rx) = channel();

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:30000") {
        println!("Connected to the server!");

        thread::spawn(move|| {
            'read_loop: loop {
                let content: String = rx.recv().unwrap();
                let res = stream.write(content.as_bytes()).unwrap();
            }
        });
    } else {
        println!("Couldn't connect to server...");
    }


    'io_in: loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        tx.send(buffer.to_owned()).unwrap();
    }
}

