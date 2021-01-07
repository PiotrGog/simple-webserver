use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

use crate::message::Message;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        return Worker {
            id: id,
            thread: Some(thread::spawn(move || loop {
                let message = receiver.lock().unwrap().recv().unwrap();

                match message {
                    Message::Execute(job) => {
                        println!("Worker {} got a job; executing.", id);
                        job();
                    }
                    Message::Terminate => break,
                }
            })),
        };
    }
}
