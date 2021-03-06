use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

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

        let get = b"GET / HTTP/1.1\r\n";
        let sleep = b"GET /sleep HTTP/1.1\r\n";

        let (status_line, file_contents) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\n", "hello.html")
        } else if buffer.starts_with(sleep) {
            thread::sleep(Duration::from_secs(20));
            ("HTTP/1.1 200 OK\r\n\n", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

        let contents = fs::read_to_string(file_contents).unwrap();
        let response = format!("{}{}", status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
