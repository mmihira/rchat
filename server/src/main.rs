mod con_worker;
use std::net::TcpListener;
use std::str;
use con_worker::ConWorker;

fn main() {

    let listener = TcpListener::bind("127.0.0.1:30000").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("new client!");
                ConWorker::spawn(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
}
