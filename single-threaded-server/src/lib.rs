use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

pub struct SingleThreadServer();

impl SingleThreadServer {
    pub fn new() -> SingleThreadServer {
        return SingleThreadServer();
    }

    pub fn start_listening(&self) {
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    }
}
