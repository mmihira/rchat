use std;
use std::{thread, time};
use std::io::{self};
use std::net::TcpStream;
use std::io::BufReader;
use std::io::BufRead;

pub struct ConWorker {
    pub hndl: std::thread::JoinHandle<()>
}

impl ConWorker {
    pub fn spawn(mut stream: TcpStream) -> ConWorker {
        ConWorker {
            hndl: thread::spawn(move || {
                let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());
                'readloop: loop {
                    let mut buffer = String::new();
                    match buffered_stream.read_line(&mut buffer) {
                        Ok(0) => {
                            println!("{:?}","Connection close");
                            break 'readloop;
                        },
                        Ok(_) => {
                            println!("{:?}", buffer);
                        }
                        Err(e) => {
                            println!("{:?}", e);
                            break 'readloop;
                        }
                    };

                }
            })
        }
    }
}
