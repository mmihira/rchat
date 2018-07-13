pub struct RegisterConnection {
    name: String
}

pub struct Message {
    content: String
}

/**
 * InterThread communication occurs entirely using
 * these possible messages.
 */
pub enum ServerMsg {
    RegisterConnection(RegisterConnection),
    Message(Message)
}

