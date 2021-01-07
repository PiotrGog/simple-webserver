use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

use crate::message::Message;
use crate::worker::Worker;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(number_of_threads: usize) -> ThreadPool {
        assert!(number_of_threads > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(number_of_threads);

        for id in 0..number_of_threads {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        return ThreadPool { workers, sender };
    }

    pub fn execute<F>(&self, fun: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(fun);
        self.sender.send(Message::Execute(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            if let Some(t) = worker.thread.take() {
                println!("Shutting down worker with id {}", worker.id);
                t.join().unwrap();
            }
        }
    }
}
