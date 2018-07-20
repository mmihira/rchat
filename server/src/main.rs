extern crate pretty_env_logger;
#[macro_use] extern crate log;

extern crate serde_json;
extern crate msg_protocol;
extern crate threadpool;
use msg_protocol::MsgProtocol;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod con_reader;
use con_reader::ConReader;

mod manager;
use manager::ManagerMsg;
use manager::manager_msg;

use std::net::TcpListener;
use std::sync::mpsc::channel;

fn main() {
    pretty_env_logger::init();
    let (manager_send, manager_receive): (Sender<ManagerMsg>, Receiver<ManagerMsg>) = channel();
    let _manager_handle = manager::Manager::spawn(manager_receive);

    info!("Server starting");

    let listener = TcpListener::bind("127.0.0.1:30000").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New Client {:?}", stream);
                ConReader::spawn(stream, manager_send.clone());
            },
            Err(e) => { /* connection failed */ }
        }
    }
}
