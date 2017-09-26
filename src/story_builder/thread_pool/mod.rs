use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
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
    unread_jobs: u64,
}

impl<I, O> ThreadPool<I, O>
where
    I: Send + 'static,
    O: Send + 'static,
{
    pub fn new(thread_count: u16, job: Arc<Job<I, O> + Send + Sync>) -> Self {
        // Create the channels to send requests and responses with the threads:
        let (request_sender, request_receiver) = channel();
        let (response_sender, response_receiver) = channel();
        // Wrap the request receiver in an Arc<Mutex> to allow use in thread
        let request_receiver = Arc::new(Mutex::new(request_receiver));
        // Spawn threads:
        let mut threads = vec![];
        for _ in 0..thread_count {
            // Grab a reference to the Job:
            let thread_job = job.clone();
            // Clone the response sender to send in the thread:
            let response_sender = response_sender.clone();
            // Clone the request receiver to send in the thread:
            let request_receiver = request_receiver.clone();
            threads.push(thread::spawn(move || loop {
                let msg = request_receiver.lock().unwrap().recv();
                match msg {
                    Ok(input) => {
                        response_sender.send(thread_job.step(input));
                    }
                    _ => {
                        return;
                    }
                };
            }));
        }

        ThreadPool {
            request_sender,
            response_receiver,
            threads,
            unread_jobs: 0,
        }
    }

    pub fn send(&mut self, value: I) {
        self.request_sender.send(value);
        self.unread_jobs += 1;
    }

    pub fn receive(&mut self) -> O {
        self.unread_jobs -= 1;
        self.response_receiver.recv().unwrap()
    }

    pub fn results_to_receive(&self) -> u64 {
        self.unread_jobs
    }

    pub fn iter(&mut self) -> ThreadPoolIterator<I, O> {
        ThreadPoolIterator {
            thread_pool: self
        }
    }
}

pub struct ThreadPoolIterator<'a, I: 'a, O: 'a>
{
    thread_pool: &'a mut ThreadPool<I, O>
}

impl <'a, I, O> Iterator for ThreadPoolIterator<'a, I, O>where
    I: Send + 'static,
    O: Send + 'static, {
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if self.thread_pool.results_to_receive() > 0 {
            Some(self.thread_pool.receive())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests;
