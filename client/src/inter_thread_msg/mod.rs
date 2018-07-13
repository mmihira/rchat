struct NewMessage {
    content: String
}

pub enum InterThreadMsg {
    NewMessage(NewMessage)
}
