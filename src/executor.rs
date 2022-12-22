use anyhow::Result;
use std::{
    any::Any,
    collections::VecDeque,
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = (
    Box<dyn FnOnce() -> Box<dyn Any + Send> + Send + 'static>,
    mpsc::Sender<Box<dyn Send>>,
);

pub enum Executor {
    ThreadPool(ThreadPool),
}

struct ThreadPool {
    threads: Vec<(thread::JoinHandle<()>, mpsc::Sender<Box<dyn Any + Send>>)>,
    queue: Arc<Mutex<VecDeque<Job>>>,
}

impl ThreadPool {
    fn new(size: usize) -> Result<ThreadPool> {
        let mut threads = Vec::with_capacity(size);
        let queue = Arc::new(Mutex::new(VecDeque::<Job>::new()));

        for _ in 0..size {
            let (tx, rx) = mpsc::channel();
            let queue_inner = queue.clone();

            let handle = thread::spawn(move || loop {
                let job = {
                    let queue = queue_inner.lock().unwrap();
                    queue.pop_front()
                };

                match job {
                    Some((task, sender)) => {
                        let result = task();
                        sender.send(Box::new(result)).unwrap();
                    }

                    None => break,
                }
            });
            threads.push((handle, tx));
        }

        Ok(ThreadPool { threads, queue })
    }

    fn execute<F>(&self, f: F) -> mpsc::Receiver<Box<dyn Send + 'static>>
    where
        F: FnOnce() -> Box<dyn Any + Send> + Send + 'static,
    {
        let (tx, rx) = mpsc::channel();

        self.queue.lock().unwrap().push_back((Box::new(f), tx));

        rx
    }
}

// #[cfg(test)]
// mod tests {
// use super::*;
// use anyhow::*;

// #[test]
// fn simple_threadpool() -> Result<()> {
// let threadpool = ThreadPool::new(4)?;
// let f = || Box::new(3 + 3);

// let six = threadpool.execute(Box::new(f));
// Ok(())
// }
// }
