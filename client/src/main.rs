extern crate msg_protocol;
use msg_protocol::MsgProtocol;
use msg_protocol::MsgProtocol::{
    NewClientRequest,
    NewClientResponse,
    TypedNewMessage,
    ServerMessage
};
extern crate serde_json;
extern crate uuid;
use uuid::Uuid;

mod read_worker;

use std::net::TcpStream;
use std::io::Write;

use std::error::Error as StdError;
use std::io::stdin;
use std::result::Result as StdResult;

use std::{thread, time};
use std::sync::mpsc::channel;

use std::process::exit;

fn main() {
    // Action channel
    let (tx, rx) = channel();

    // Connect to server
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:30000") {
        println!("Connected to the server!");

        let mut write_stream_clone = stream.try_clone().unwrap();
        let mut read_stream_clone = stream.try_clone().unwrap();

        let _write_thread = thread::spawn(move|| {

            // First write
            let name = Uuid::new_v4().to_string();
            let client  = NewClientRequest(name);
            let serialized = serde_json::to_string(&client).unwrap();
            write_stream_clone.write(serialized.as_bytes()).unwrap();

            // First message must be name accepted
            let server_acceptance: MsgProtocol = rx.recv().unwrap();

            if let NewClientResponse(result) = server_acceptance {
                println!("{:?}", "Server allowed access!");
            } else {
                println!("{:?}", "Server denied access!");
                return 0;
            }

            'write_loop: loop {
                let msg: MsgProtocol = rx.recv().unwrap();
                match msg {
                    TypedNewMessage(ref content) => {
                        let res = write_stream_clone.write(content.as_bytes()).unwrap();
                    },
                    ServerMessage(ref content) => {
                        println!("{:?}", content);
                    },
                    _ => {}
                }
            }
        });

        // Read thread
        let _read_thread = read_worker::ReadWorker::spawn(
             read_stream_clone.try_clone().unwrap(),
            tx.clone()
        );
    } else {
        println!("Couldn't connect to server...");
    }

    // IO loop
    'io_in: loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();

        tx.send(TypedNewMessage(buffer.to_owned())).unwrap();
    }
}

