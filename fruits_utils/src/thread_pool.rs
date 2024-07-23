use std::{
    sync::{
        mpsc::{
            Sender,
            self,
            Receiver
        },
        Arc,
        Mutex
    },
    thread,
};

type DefaultJob = Box<dyn FnOnce() + Send>;

pub trait Job : 'static + Send {
    fn execute(self);
}

impl Job for Box<dyn FnOnce() + Send> {
    fn execute(self) {
        self()
    }
}

enum Message<J: Job> {
    JobRequest(J),
    TerminateRequest,
}

// todo: 'static?
pub struct ThreadPool<J: Job = DefaultJob>
{
    threads: Box<[Option<thread::JoinHandle<()>>]>,
    message_sender: Sender<Message<J>>
}

impl<J: Job> ThreadPool<J> {
    pub fn new(threads_count: usize) -> Self {
        assert!(threads_count > 0);

        let mut threads = Vec::with_capacity(threads_count);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver)); 

        for id in 0..threads_count {
            threads.push(Some(ThreadPool::run_worker(id, Arc::clone(&receiver))));
        }
        
        Self {
            threads: threads.into_boxed_slice(),
            message_sender: sender,
        }
    }

    pub fn push_job(&self, job: J) {
        self.message_sender.send(Message::JobRequest(job)).unwrap();
    }

    fn run_worker(_id: usize, message_receiver: Arc<Mutex<Receiver<Message<J>>>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let message = {
                    message_receiver.lock().unwrap().recv().unwrap()
                };

                match message {
                    Message::TerminateRequest => break,
                    Message::JobRequest(job) => job.execute(),
                }
            }
        })
    }
}

impl<J: Job> Drop for ThreadPool<J> {
    fn drop(&mut self) {
        for _ in 0..self.threads.len() {
            self.message_sender.send(Message::TerminateRequest).unwrap();
        }

        for thread in self.threads.iter_mut() {
            thread.take().unwrap().join().unwrap();
        }
    }
}
