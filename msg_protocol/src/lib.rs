#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewClientResponse {
    pub id: String,
    pub response: bool
}

/**
 * Messages sent between the client and the server
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum MsgProtocol {
    // Client-Server
    NewClientRequest(String),
    NewClientResponse(NewClientResponse),

    RequestRoomList(bool),
    ResponseRoomList(Vec<String>),

    LeaveCurrentRoom(bool),

    TypedNewMessage(String),
    ToClientMsgFromRoom(String)
}

impl MsgProtocol {
    pub fn parse_msg(msg: &String) -> MsgProtocol  {
        serde_json::from_str(&msg.trim()).unwrap()
    }

    pub fn to_string(msg: &MsgProtocol) -> String {
        serde_json::to_string(msg).unwrap() + "\n"
    }
}


