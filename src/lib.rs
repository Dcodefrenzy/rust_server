use std::{thread, sync::{mpsc, Mutex, Arc}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Mesage>
}


type Job = Box<dyn FnOnce() + Send + 'static>;
enum Mesage {
    NewJob(Job),
    Terminate
}

impl ThreadPool {
    /// Create a new ThreadPool
    /// 
    /// The size is the number of threads in the pool.
    /// 
    /// # Panics 
    /// 
    /// Th 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, recever) = mpsc::channel();
        let recever = Arc::new(Mutex::new(recever));
        

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&recever)));
        }
        ThreadPool {workers, sender}
    }
    pub fn execute<F>(&self, f: F)
    where 
    F: FnOnce() + Send + 'static 
    {
        let job = Box::new(f);
        self.sender.send(Mesage::NewJob(job)).unwrap();
    }
}


impl Drop for ThreadPool {
    fn drop(&mut self) {

        for _ in &self.workers {
            self.sender.send(Mesage::Terminate).unwrap()
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

        }
    }
}
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    fn new(id:usize, recever: Arc<Mutex<mpsc::Receiver<Mesage>>>) -> Worker {
        let thread = thread::spawn(move || loop{
            let message = recever.lock().unwrap().recv().unwrap();
            match message {
                Mesage::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Mesage::Terminate => {
                    println!("Worker {} got a job; executing.", id);
                    break;

                }
            }
        });
        Worker { id, thread:Some(thread) }
    }
}
