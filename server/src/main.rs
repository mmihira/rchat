mod con_worker;
mod con_writer;
use con_writer::{
    ConWriter,
    ConWriterMsg,
    NewSocket};
use std::net::TcpListener;
use std::str;
use con_worker::ConWorker;

use std::sync::mpsc::channel;

use std::collections::HashMap;

fn main() {
    // Writer Channel
    let (writer_send, writer_receive) = channel();
    let z = ConWriter::spawn(writer_receive);

    let listener = TcpListener::bind("127.0.0.1:30000").unwrap();

    // let mut = HashMap::new();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new client!");
                writer_send.send(ConWriterMsg::NewSocket(NewSocket{
                    id: "Hello".to_string(),
                    socket: stream.try_clone().unwrap()
                }));
                ConWorker::spawn(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
