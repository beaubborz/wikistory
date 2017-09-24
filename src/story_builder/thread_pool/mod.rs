use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

pub trait Job<I, O> {
    /// This function is called every time the thread receives a message from the thread pool.
    fn step(&self, input: I) -> O;
}

pub struct ThreadPool<I, O> {
    request_sender: Sender<I>,
    response_receiver: Receiver<O>,
    threads: Vec<JoinHandle<()>>,
}

impl <I, O> ThreadPool<I, O> where
     I: Send + 'static, O: Send + 'static {
    pub fn new(thread_count: u16, job: Arc<Job<I, O> + Send + Sync>) -> Self {
        // Create the channels to send requests and responses with the threads:
        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();
        // Grab a reference to the Job:
        let thread_job = job.clone();
        // Spawn threads:
        let mut threads = vec![];
        threads.push(thread::spawn(move || {
            response_sender.send(thread_job.step(request_receiver.recv().unwrap()));
        }));

        ThreadPool {
            request_sender,
            response_receiver,
            threads,
        }
    }

    pub fn send(&self, value: I) {
        self.request_sender.send(value);
    }

    pub fn receive(&self) -> O {
        self.response_receiver.recv().unwrap()
    }
}

#[cfg(test)]
mod tests;
