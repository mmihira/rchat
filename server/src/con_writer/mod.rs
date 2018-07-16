use std;
use std::{thread};
use std::sync::mpsc::Receiver;
use std::net::TcpStream;

pub struct NewSocket {
    pub id: String,
    pub socket: TcpStream
}

pub enum ConWriterMsg {
    NewSocket(NewSocket)
}

pub struct ConWriter {
    pub hndl: std::thread::JoinHandle<()>
}

impl ConWriter {
    pub fn spawn(recv_channel: Receiver<ConWriterMsg> ) -> ConWriter {
        ConWriter{
            hndl: thread::spawn(move || {
                'main_loop: loop {
                    let msg: ConWriterMsg = recv_channel.recv().unwrap();
                    match msg {
                        ConWriterMsg::NewSocket(_) => {
                            println!("{:?}", "Received new socket");
                        },
                        _ => {}
                    }
                }
            })
        }
    }
}
