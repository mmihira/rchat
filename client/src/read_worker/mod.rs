use msg_protocol::MsgProtocol;

use std;
use std::net::TcpStream;
use std::{thread};
use std::sync::mpsc::Sender;
use std::io::BufReader;
use std::io::BufRead;

pub struct ReadWorker {
    pub hndl: std::thread::JoinHandle<()>
}

/**
 * Reads messages from server and places them in an action que
 */
impl ReadWorker {
    pub fn spawn(stream: TcpStream, send_channel: Sender<MsgProtocol>) -> ReadWorker {
        ReadWorker {
            hndl: thread::spawn(move || {
                let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());
                'readloop: loop {
                    let mut buffer = String::new();
                    match buffered_stream.read_line(&mut buffer) {
                        Ok(0) => {
                            println!("{:}","Connection closed.");
                            break 'readloop;
                        },
                        Ok(_) => {
                            send_channel.send(MsgProtocol::parse_msg(&buffer)).unwrap();
                        },
                        Err(e) => {
                            println!("Error in ReadWorker {:}", e);
                            break 'readloop;
                        }
                    };

                }
            })
        }
    }
}
