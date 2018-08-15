extern crate pretty_env_logger;
extern crate clap;
#[macro_use] extern crate log;

extern crate serde_json;
extern crate msg_protocol;
extern crate threadpool;

use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

mod con_reader;
use con_reader::ConReader;

mod manager;
use manager::ManagerMsg;

use std::net::TcpListener;
use std::sync::mpsc::channel;

use clap::{Arg, App};

fn main() {
    pretty_env_logger::init();

    let console_args = App::new("Chat server")
        .about("Chat server")
        .arg(Arg::with_name("host")
            .short("h")
            .help("Server address")
            .takes_value(true))
        .get_matches();

    let server_host = console_args.value_of("host").unwrap_or("127.0.0.1:30000");

    let (manager_send, manager_receive): (Sender<ManagerMsg>, Receiver<ManagerMsg>) = channel();
    let _manager_handle = manager::Manager::spawn(manager_receive);

    info!("Server starting");

    let listener = TcpListener::bind(server_host).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                info!("New Client {:?}", stream);
                ConReader::spawn(stream, manager_send.clone());
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}
