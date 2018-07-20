use std;
use std::{thread};
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
use std::sync::mpsc::Sender;

use msg_protocol::MsgProtocol;

use manager;
use manager::ManagerMsg;
use manager::manager_msg;

pub struct ConReader {
    pub hndl: std::thread::JoinHandle<()>
}

impl ConReader {
    pub fn spawn(mut stream: TcpStream, manager_send: Sender<ManagerMsg>) -> ConReader {
        ConReader {
            hndl: thread::spawn(move || {
                info!("ConReader starting for {:?}", stream);
                let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());

                // First message expected to be NewClientRequest
                let mut nc_buffer = String::new();
                match buffered_stream.read_line(&mut nc_buffer) {
                    Ok(0) => {
                        info!("Read connection closed for {:?}", stream);
                        return
                    },
                    Ok(_) => {
                        // It is possible that the socket creation msg will be placed
                        // after a send msg. So really we need the socket to be locked.
                        // Or move this into the worker thread and have that thread wait
                        // until the manager adds the socket into the hashmap !

                        let new_client_request = MsgProtocol::parse_msg(&nc_buffer);
                        if let MsgProtocol::NewClientRequest(name) = new_client_request {
                            let client_name = name.to_string();
                            manager_send.send(ManagerMsg::NewSocket(manager_msg::NewSocket{
                                id: name,
                                socket: stream.try_clone().unwrap()
                            }));

                            /**
                             * Main read loop after first message
                             */
                            Self::read_loop(client_name, &mut buffered_stream, manager_send);
                        } else {
                            info!("Expected a NewClientRequest initially from {:?}", stream);
                            return
                        }

                    }
                    Err(e) => {
                        info!("Error: {:?}", e);
                        return
                    }
                }
            })
        }
    }

    fn read_loop(client_name: String, buffered_stream: &mut BufReader<TcpStream>, manager_send: Sender<ManagerMsg>) {
        'readloop: loop {
            let mut buffer = String::new();
            match buffered_stream.read_line(&mut buffer) {
                Ok(0) => {
                    info!("Read connection closed for {:?}", buffered_stream);
                    break 'readloop;
                },
                Ok(_) => {
                    let msg: MsgProtocol = MsgProtocol::parse_msg(&buffer);
                    manager_send.send(
                        ManagerMsg::ProtocolWrapper(manager_msg::ProtocolWrapper{
                            client_name: client_name.to_string(),
                            msg_protocol: msg
                        })
                    );
                    },
                Err(e) => {
                    info!("Error: {:?}", e);
                    break 'readloop;
                }
            };
        }
    }
}
