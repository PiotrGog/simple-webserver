use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

pub struct MultiThreadServer();

impl MultiThreadServer {
    pub fn new() -> MultiThreadServer {
        return MultiThreadServer();
    }

    pub fn start_listening(&self) {
        let thread_pool = ThreadPool::new(4);
        let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            thread_pool.execute(|| {
                Self::handle_connection(stream);
            });
        }
    }

    fn handle_connection(mut stream: TcpStream) {
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

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    fn new(number_of_threads: usize) -> ThreadPool {
        assert!(number_of_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(number_of_threads);

        for id in 0..number_of_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        return ThreadPool { workers, sender };
    }

    fn execute<F>(&self, fun: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(fun);
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                println!("Shutting down worker with id {}", worker.id);
                t.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        return Worker {
            id: id,
            thread: Some(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();

                println!("Worker {} got a job; executing.", id);

                job();
            })),
        };
    }
}
