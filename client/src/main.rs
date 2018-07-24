extern crate msg_protocol;
extern crate regex;
extern crate names;
extern crate colored;

use colored::*;
use regex::Regex;
use names::{Generator, Name};

use msg_protocol::MsgProtocol;
use msg_protocol::MsgProtocol::{
    NewClientRequest,
    NewClientResponse,
    RequestTypedNewMessage,
    ResponseTypedMessage,
    RequestCreateRoom,
    ResponseCreateRoom,
    RequestJoinRoom,
    ResponseJoinRoom,
    RequestRoomList,
    ResponseRoomList
};
extern crate serde_json;
mod read_worker;

use std::net::TcpStream;
use std::io::Write;

use std::io::stdin;

use std::{thread};
use std::sync::mpsc::channel;

fn main() {
    // Action channel
    let (tx, rx) = channel();

    // Connect to server
    if let Ok(stream) = TcpStream::connect("127.0.0.1:30000") {
        println!("{}", "::Connected to the server!".green());

        let mut write_stream_clone = stream.try_clone().unwrap();
        let mut read_stream_clone = stream.try_clone().unwrap();

        let _write_thread = thread::spawn(move|| {
            let mut generator = Generator::with_naming(Name::Plain);

            // First write
            let name = generator.next().unwrap();
            let client  = NewClientRequest(name);
            let serialized = MsgProtocol::to_string(&client);
            write_stream_clone.write(serialized.as_bytes()).unwrap();

            // First message must be name accepted
            let server_acceptance: MsgProtocol = rx.recv().unwrap();

            if let NewClientResponse(_) = server_acceptance {
                println!("{}", "::Server allowed access!".green());
                println!("{}", "\nChat commands available: ".cyan());
                println!("{}", "/create <room_name> : Create a new room".cyan());
                println!("{}", "/join <room_name> : Join a new room".cyan());
                println!("{}", "/list rooms : List all rooms\n".cyan());
            } else {
                println!("{}", "::Server denied access!".green());
                return 0;
            }

            'write_loop: loop {
                let msg: MsgProtocol = rx.recv().unwrap();
                match msg {
                    RequestTypedNewMessage(ref _content) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    ResponseTypedMessage(ref msg) => {
                        println!("{:}: {:}", msg.client_name, msg.msg.trim().bold());
                    },
                    RequestRoomList(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    RequestCreateRoom(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    RequestJoinRoom(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    }
                    ResponseCreateRoom(result) => {
                        if result {
                            println!("{:}", "::Created room".green());
                        } else {
                            println!("{:}", "::Room allready exists".green());
                        }
                    },
                    ResponseJoinRoom(result) => {
                        if result {
                            println!("{:}", "::Joined room".green());
                        } else {
                            println!("{:}", "::Could not join room".green());
                        }
                    },
                    ResponseRoomList(ref list) => {
                        println!("{:}","::Rooms available are".green());
                        for room_name in list.iter() {
                            println!("- {:}", room_name.green());
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
        println!("{}", "::Couldn't connect to server...".green());
    }

    // IO loop
    'io_in: loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        let re = Regex::new(r"/(.*) (.*)").unwrap();

        if buffer.len() > 0 {
            if buffer.starts_with(r"/") {
                if let Some(capture) = re.captures(&buffer) {
                    match &capture[1] {
                        "join" => tx.send(RequestJoinRoom(capture[2].to_string())).unwrap(),
                        "create" => tx.send(RequestCreateRoom(capture[2].to_string())).unwrap(),
                        "list" => tx.send(RequestRoomList(true)).unwrap(),
                        _ => println!("{:}", "::Invalid command".green())
                    }
                } else {
                    println!("{:}", "Not captured");
                }
            } else {
                tx.send(RequestTypedNewMessage(buffer.to_owned())).unwrap();
            }
        }
    }
}
