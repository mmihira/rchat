extern crate names;
extern crate msg_protocol;
extern crate regex;
extern crate serde_json;
extern crate tui;
extern crate termion;
extern crate clap;

mod read_worker;
mod app_ui;
use app_ui::AppMsg;

use std::net::TcpStream;
use std::io;
use std::io::Write;
use std::{thread};
use std::sync::mpsc::channel;

use tui::Terminal;
use tui::backend::RawBackend;
use tui::widgets::{Widget, Block, Borders, Paragraph};
use tui::layout::{Group, Size, Direction};
use tui::style::{Color, Style};

use termion::event;
use termion::input::TermRead;

use regex::Regex;
use names::{Generator, Name};

use msg_protocol::MsgProtocol;
use msg_protocol::MsgProtocol::{
    NewClientRequest,
    NewClientResponse,
    RequestTypedNewMessage,
    ResponseTypedMessage,
    RequestCreateRoom,
    ResponseCreateRoom,
    RequestJoinRoom,
    ResponseJoinRoom,
    RequestRoomList,
    ResponseRoomList
};

use clap::{Arg, App};

fn init() -> Result<Terminal<RawBackend>, io::Error> {
    let backend = RawBackend::new()?;
    Terminal::new(backend)
}

fn main() {
    let console_args = App::new("My Super Program")
        .about("Chat client")
        .arg(Arg::with_name("host")
            .short("h")
            .help("Server address")
            .required(true)
            .takes_value(true))
        .arg(Arg::with_name("name")
            .short("n")
            .help("User name")
            .takes_value(true))
      .get_matches();

    let server_host = console_args.value_of("host").unwrap_or("127.0.0.1:30000");
    let mut generator = Generator::with_naming(Name::Plain);
    let tmp_name = generator.next().unwrap();
    let name = console_args.value_of("name").unwrap_or(&tmp_name);
    let name_copy = name.to_string();

    let (msg_protocol_tx, msg_protocol_rx) = channel();
    let (app_msg_tx, app_msg_rx) = channel();
    let app_msg_tx_writer = app_msg_tx.clone();

    // Connect to server
    if let Ok(stream) = TcpStream::connect(server_host) {
        app_msg_tx.send(AppMsg::Info("::Connected to the server!".to_string())).unwrap();
        let app_msg_tx_wp = app_msg_tx.clone();

        let mut write_stream_clone = stream.try_clone().unwrap();
        let mut read_stream_clone = stream.try_clone().unwrap();

        let _write_thread = thread::spawn(move|| {

            // First write
            let client = NewClientRequest(name_copy);
            let serialized = MsgProtocol::to_string(&client);
            write_stream_clone.write(serialized.as_bytes()).unwrap();

            // First message must be name accepted
            let server_acceptance: MsgProtocol = msg_protocol_rx.recv().unwrap();

            if let NewClientResponse(_) = server_acceptance {
                app_msg_tx_wp.send(AppMsg::Info("::Server allowed access!".to_string())).unwrap();
                app_msg_tx_wp.send(AppMsg::Info("::Chat commands availbale: ".to_string())).unwrap();
                app_msg_tx_wp.send(AppMsg::Info(" /create <room_name> : Create a new room".to_string())).unwrap();
                app_msg_tx_wp.send(AppMsg::Info(" /join <room_name>   : Join a new room".to_string())).unwrap();
                app_msg_tx_wp.send(AppMsg::Info(" /list rooms         : list all rooms".to_string())).unwrap();
            } else {
                app_msg_tx_writer.send(AppMsg::Error("::Server denied access!".to_string())).unwrap();
                return 0;
            }

            'write_loop: loop {
                let msg: MsgProtocol = msg_protocol_rx.recv().unwrap();
                match msg {
                    RequestTypedNewMessage(ref _content) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    ResponseTypedMessage(msg) => {
                        app_msg_tx_wp.send(AppMsg::ChatMsg(msg)).unwrap();
                    },
                    RequestRoomList(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    RequestCreateRoom(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    },
                    RequestJoinRoom(_) => {
                        let _res = write_stream_clone.write(
                            MsgProtocol::to_string(&msg).as_bytes()
                        ).unwrap();
                    }
                    ResponseCreateRoom(result) => {
                        if result {
                            app_msg_tx_wp.send(AppMsg::Info("::Created room".to_string())).unwrap();
                        } else {
                            app_msg_tx_wp.send(AppMsg::Info("::Room allready exists".to_string())).unwrap();
                        }
                    },
                    ResponseJoinRoom(result) => {
                        if result {
                            app_msg_tx_wp.send(AppMsg::Info("::Joined room".to_string())).unwrap();
                        } else {
                            app_msg_tx_wp.send(AppMsg::Info("::Could not join room".to_string())).unwrap();
                        }
                    },
                    ResponseRoomList(ref list) => {
                        app_msg_tx_wp.send(AppMsg::Info("::Rooms available are".to_string())).unwrap();
                        for room_name in list.iter() {
                            app_msg_tx_wp.send(AppMsg::Info(
                                format!("- {:}", room_name).to_string()
                            )).unwrap();
                        }
                    }
                    _ => {}
                }
            }
        });

        // Read thread
        let _read_thread = read_worker::ReadWorker::spawn(
             read_stream_clone.try_clone().unwrap(),
            msg_protocol_tx.clone()
        );
    } else {
        app_msg_tx.send(AppMsg::Info("::Coudn't connect to the server!".to_string())).unwrap();
        println!("{}", "Coudn't connect to the server");
        return;
    }

    let _io_thread = thread::spawn(move|| {
        // IO loop
        let app_msg_tx_send = app_msg_tx.clone();
        'io_in: loop {
            let re = Regex::new(r"/(.*) (.*)").unwrap();

            let mut current_line_buffer = String::new();
            let stdin = io::stdin();
            for character in stdin.keys() {
                let evt = character.unwrap();

                match evt {
                    event::Key::Char('\n') => {
                        if current_line_buffer.len() > 0 && current_line_buffer.starts_with(r"/") {
                            let buffer_copy = current_line_buffer.to_string() + "\n";
                            if let Some(capture) = re.captures(&buffer_copy) {
                                match &capture[1] {
                                    "join" => msg_protocol_tx.send(RequestJoinRoom(capture[2].to_string())).unwrap(),
                                    "create" => msg_protocol_tx.send(RequestCreateRoom(capture[2].to_string())).unwrap(),
                                    "list" => msg_protocol_tx.send(RequestRoomList(true)).unwrap(),
                                    "exit" => break 'io_in,
                                    _ => app_msg_tx_send.send(AppMsg::Error("::Invalid command".to_string())).unwrap()
                                }
                            }
                        } else {
                            msg_protocol_tx.send(RequestTypedNewMessage(current_line_buffer.to_owned())).unwrap();
                        }

                        app_msg_tx_send.send(AppMsg::NewLine).unwrap();
                        current_line_buffer.clear();
                    },
                    event::Key::Backspace => {
                        if current_line_buffer.len() > 0 {
                            let len_to_trun = current_line_buffer.len() - 1;
                            current_line_buffer.truncate(len_to_trun);
                            app_msg_tx_send.send(AppMsg::BackSpace).unwrap();
                        }
                    },
                    event::Key::Char(some_char) => {
                        current_line_buffer.push(some_char);
                        app_msg_tx_send.send(AppMsg::Char(some_char.to_string())).unwrap();
                    },
                    _ => ()
                }
            }
        }
    });

    let mut app_ui = app_ui::AppUi::new();
    let mut terminal = init().expect("Failed initialization");
    terminal.clear().unwrap();

    'ui_loop: loop {
        let app_msg = app_msg_rx.recv().unwrap();

        // YOU NEED TO HANDLE
        // The case where the input length is currently equal to len of the screen size -4
        // You'll have to wrap the lines or something ....
        // Could move this logic into the UiApp
        // Also need to catch window resize eventually ....
        match app_msg {
            AppMsg::Char(some_char) => {
                app_ui.input.push_str(&some_char);
            },
            AppMsg::BackSpace => {
                app_ui.backspace()
            }
            AppMsg::Info(info) => {
                app_ui.messages.push(format!("{{fg=magenta {} }}", info));
            },
            AppMsg::ChatMsg(msg_protocol::MsgResponse{client_name, msg}) => {
                app_ui.messages.push(format!("{{fg=green {}: {}}}", client_name, msg));
            },
            AppMsg::NewLine => {
                let drained: String = app_ui.input.drain(..).collect();
                app_ui.messages.push(format!("{{fg=white {}: {}}}", name, drained));
            },
            AppMsg::Error(_) => {
                let copy: String = app_ui.input.drain(..).collect();
                app_ui.messages.push(
                    format!("{{fg=red ::Invalid command {} }}", copy)
                    );
            }
        }

        draw_ui(&mut terminal, &app_ui);
    }
}

fn draw_ui(mut terminal: &mut tui::Terminal<RawBackend>, app_ui: &app_ui::AppUi) {
    let size = terminal.size().unwrap();

    Group::default()
        .direction(Direction::Vertical)
        .margin(1)
        .sizes(&[Size::Percent(80), Size::Percent(20)])
        .render(&mut terminal, &size, |t, chunks| {
            let input_text = format!("{{fg=white {}}}", app_ui.input);

            Paragraph::default()
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .text(&input_text)
                .render(t, &chunks[1]);

            Paragraph::default()
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Input"))
                .text(&app_ui.get_messages_for_display())
                .render(t, &chunks[0]);

            Block::default()
                .borders(Borders::ALL)
                .render(t, &chunks[1]);
        });

    terminal.draw().unwrap();
}
