#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

/**
 * Messages sent between the client and the server
 */
#[derive(Serialize, Deserialize, Debug)]
pub enum MsgProtocol {
    // Client-Server
    NewClientRequest(String),
    NewClientResponse(bool),

    // Client
    TypedNewMessage(String),
    ServerMessage(String)

    // Server
}


