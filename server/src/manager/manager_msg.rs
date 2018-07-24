use std::net::TcpStream;
use msg_protocol::MsgProtocol;

pub struct NewSocket {
    pub id: String,
    pub socket: TcpStream
}

pub struct ProtocolWrapper {
    pub client_name: String,
    pub msg_protocol: MsgProtocol
}

pub enum ManagerMsg {
    NewSocket(NewSocket),
    ProtocolWrapper(ProtocolWrapper)
}
