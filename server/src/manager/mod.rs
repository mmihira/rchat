extern crate serde_json;

use std;
use std::{thread};
use std::sync::mpsc::Receiver;
use std::net::TcpStream;
use std::io::Write;
use std::collections::HashMap;
use threadpool::ThreadPool;

pub mod manager_msg;
pub use self::manager_msg::ManagerMsg;
use msg_protocol;
use msg_protocol::MsgProtocol;
mod chat_rooms;

const NOWORKERS: usize = 4;

struct ManagerInternal {
    pub chat_rooms: chat_rooms::ChatRooms,
    pub thread_pool: ThreadPool,
    pub socket_map: HashMap<String, TcpStream>
}

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

        let mut internal = ManagerInternal {
            socket_map: HashMap::new(),
            thread_pool: ThreadPool::new(self::NOWORKERS),
            chat_rooms: chat_rooms::ChatRooms::new()
        };

        'main_loop: loop {
            let channel_msg = recv_channel.recv().unwrap();

            match channel_msg {
                ManagerMsg::NewSocket(manager_msg::NewSocket {id: s_id, socket: soc}) => {
                    let mut new_socket = soc.try_clone().unwrap();
                    let id_clone = s_id.clone();
                    // Keep track of the socket
                    internal.socket_map.insert(s_id, soc);

                    // Insert user into default_room
                    internal.chat_rooms.insert_client_into_room(
                        &id_clone,
                        chat_rooms::DEFAULT_ROOM
                    );

                    internal.thread_pool.execute(move || {
                        let response = MsgProtocol::NewClientResponse(
                            msg_protocol::NewClientResponse {
                                id: id_clone,
                                response: true
                            }
                        );
                        new_socket.write(MsgProtocol::to_string(&response).as_bytes()).unwrap();
                    });
                },
                ManagerMsg::ProtocolWrapper(protocol_wrapper) => {
                    Manager::handle_msg_protocol(&mut internal, &protocol_wrapper);
                }
            }
        }
    }

    fn handle_msg_protocol(internal: &mut ManagerInternal, msg_protocol_wrapper: &manager_msg::ProtocolWrapper) {
        let &manager_msg::ProtocolWrapper{ref client_name, ref msg_protocol} = msg_protocol_wrapper;

        match msg_protocol {
            &MsgProtocol::TypedNewMessage(ref msg) => {
                let msg_to_write = MsgProtocol::ToClientMsgFromRoom(msg.to_string());
                let serialized = MsgProtocol::to_string(&msg_to_write);

                let client_chat_pariticipants = internal.chat_rooms.clients_room_pariticipants(client_name);

                for name in client_chat_pariticipants {
                    let mut client_socket: &TcpStream = internal.socket_map.get(&name).unwrap();
                    let mut socket_clone = client_socket.try_clone().unwrap();
                    let msg_copy = serialized.clone();
                    internal.thread_pool.execute(move || {
                        socket_clone.write(msg_copy.as_bytes()).unwrap();
                    });
                }
            },
            &MsgProtocol::LeaveCurrentRoom(_) => {
                let res = internal.chat_rooms.remove_client_from_room(&client_name);
                info!("Removed {:?} from remove with result {:?}", client_name, res);
            },
            &MsgProtocol::RequestRoomList(_) => {
                let rooms: Vec<String> = internal
                    .chat_rooms
                    .rooms
                    .keys()
                    .map(|v| v.to_string())
                    .collect();
                let mut client_socket: &TcpStream = internal.socket_map.get(client_name).unwrap();

                let msg_to_write = MsgProtocol::ResponseRoomList(rooms);
                let serialized = MsgProtocol::to_string(&msg_to_write);
                let mut socket_clone = client_socket.try_clone().unwrap();
                internal.thread_pool.execute(move || {
                    socket_clone.write(serialized.as_bytes()).unwrap();
                });
            }
            _ => info!("Unmatched message {:?}", msg_protocol)
        }
    }
}

