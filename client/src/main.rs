extern crate msg_protocol;
extern crate regex;
use regex::Regex;

use msg_protocol::MsgProtocol;
use msg_protocol::MsgProtocol::{
    NewClientRequest,
    NewClientResponse,
    TypedNewMessage,
    ToClientMsgFromRoom,
    RequestRoomList,
    ResponseRoomList
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
            let serialized = MsgProtocol::to_string(&client);
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
                        let res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    ToClientMsgFromRoom(ref msg) => {
                        println!("Ping back {:?}", msg);
                    },
                    RequestRoomList(_) => {
                        let res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    ResponseRoomList(ref list) => {
                        println!("{:?}","Rooms available are");
                        for room_name in list.iter() {
                            println!("- {:?}", room_name);
                        }
                    }
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
        let re = Regex::new(r"/(.*) (.*)").unwrap();

        if (buffer.len() > 0) {
            if(buffer.starts_with(r"/")) {
                if let Some(capture) = re.captures(&buffer) {
                    println!("{:?}",capture);
                    match &capture[1] {
                        "join" => println!("{:?}", "valid"),
                        "list" => tx.send(RequestRoomList(true)).unwrap(),
                        _ => println!("{:?}", "Invalid command")
                    }
                } else {
                    println!("{:?}", "Not captured");
                }
            } else {
                tx.send(TypedNewMessage(buffer.to_owned())).unwrap();
            }
        }
    }
}



