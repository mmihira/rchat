extern crate serde_json;

use std;
use std::{thread};
use std::sync::mpsc::Receiver;
use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;

pub mod manager_msg;
pub use self::manager_msg::ManagerMsg;

use msg_protocol;
use msg_protocol::MsgProtocol;

use threadpool::ThreadPool;

const NOWORKERS: usize = 4;

pub struct Manager {
    pub hndl: std::thread::JoinHandle<()>
}

impl Manager {
    pub fn spawn(recv_channel: Receiver<ManagerMsg>) -> Manager {
        Manager {
            hndl: thread::spawn(|| {
                &Manager::thread_process(recv_channel);
            })
        }
    }

    pub fn thread_process(recv_channel: Receiver<ManagerMsg>) {
        info!("Manager Started");
        let mut socket_map: HashMap<String, TcpStream> = HashMap::new();

        let pool = ThreadPool::new(self::NOWORKERS);

        'main_loop: loop {
            let msg = recv_channel.recv().unwrap();

            match msg {
                ManagerMsg::NewSocket(manager_msg::NewSocket {id: s_id, socket: soc}) => {
                    let mut new_socket = soc.try_clone().unwrap();
                    let id_clone = s_id.clone();
                    socket_map.insert(s_id, soc);
                    pool.execute(move || {
                        let response = MsgProtocol::NewClientResponse(msg_protocol::NewClientResponse {
                            id: id_clone,
                            response: true
                        });

                        new_socket.write(MsgProtocol::to_string(&response).as_bytes()).unwrap();
                    });
                },
                ManagerMsg::ProtocolWrapper(protocol_wrapper) => {
                    Manager::handle_msg_protocol(&socket_map, &protocol_wrapper);
                }
            }
        }

        // 'main_loop: loop {
        // }

        /*
         * - Will wait for messages on it channel
         * - Will put in write messages to the writer thread pool
         * - Keep track of sockets to write to
         * - Handle messages from the readers thread
         */
    }

    fn handle_msg_protocol(socket_map: &HashMap<String, TcpStream>, msg_protocol_wrapper: &manager_msg::ProtocolWrapper) {

        let &manager_msg::ProtocolWrapper{ref client_name, ref msg_protocol} = msg_protocol_wrapper;

        match msg_protocol {
            // &TypedNewMessage(ref msg) => {
            //     // For now ping messages back
            //     let mut client_socket: &TcpStream = socket_map.get(id).unwrap();
            //     let serialized = MsgProtocol::to_string(msg_protocol).as_bytes();
            //     client_socket.write(serialized).unwrap();
            // },
            &MsgProtocol::TypedNewMessage(ref msg) => {
                let mut client_socket: &TcpStream = socket_map.get(client_name).unwrap();
                let msg_to_write = MsgProtocol::ToClientMsgFromRoom(msg.to_string());

                let serialized = MsgProtocol::to_string(&msg_to_write);
                client_socket.write(serialized.as_bytes()).unwrap();
            },
            _ => {println!("{:?}", "Hello" );},
        }
    }
}

