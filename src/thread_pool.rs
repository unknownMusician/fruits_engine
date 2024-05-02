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

enum Message<Job> {
    JobRequest(Job),
    TerminateRequest,
}

// todo: 'static?
pub struct ThreadPool<Job: 'static + Send = DefaultJob>
{
    threads: Box<[Option<thread::JoinHandle<()>>]>,
    message_sender: Sender<Message<Job>>
}

impl<Job: 'static + Send> ThreadPool<Job> {
    pub fn new(threads_count: usize, job_executor: impl 'static + Fn(Job) + Send + Clone) -> Self {
        assert!(threads_count > 0);

        let mut threads = Vec::with_capacity(threads_count);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver)); 

        for id in 0..threads_count {
            threads.push(Some(ThreadPool::run_worker(id, Box::new(job_executor.clone()), Arc::clone(&receiver))));
        }
        
        Self {
            threads: threads.into_boxed_slice(),
            message_sender: sender,
        }
    }

    pub fn push_job(&self, f: Job) {
        self.message_sender.send(Message::<Job>::JobRequest(f)).unwrap();
    }

    fn run_worker(_id: usize, job_executor: Box<impl 'static + Fn(Job) + Send>, message_receiver: Arc<Mutex<Receiver<Message<Job>>>>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            loop {
                let message = {
                    message_receiver.lock().unwrap().recv().unwrap()
                };

                match message {
                    Message::TerminateRequest => break,
                    Message::JobRequest(job) => job_executor(job),
                }
            }
        })
    }
}

impl<Job: Send> Drop for ThreadPool<Job> {
    fn drop(&mut self) {
        for _ in 0..self.threads.len() {
            self.message_sender.send(Message::TerminateRequest).unwrap();
        }

        for thread in self.threads.iter_mut() {
            thread.take().unwrap().join().unwrap();
        }
    }
}
