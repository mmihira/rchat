use msg_protocol;

#[derive(Debug)]
pub enum AppMsg {
    Info(String),
    Error(String),
    Char(String),
    NewLine,
    BackSpace,
    ChatMsg(msg_protocol::MsgResponse)
}

pub struct AppUi {
    pub input: String,
    pub messages: Vec<String>
}


impl AppUi {
    pub fn new() -> AppUi {
        AppUi {
            input: String::new(),
            messages: Vec::new()
        }
    }

    pub fn get_messages_for_display(&self, height: usize) -> String {
        let start = if height > self.messages.len() { 0 } else { self.messages.len() - height };
        self.messages[start..self.messages.len()].join("\n")
    }

    pub fn backspace(&mut self) {
        let len_to_trun = self.input.len() - 1;
        self.input.truncate(len_to_trun);
    }
}
