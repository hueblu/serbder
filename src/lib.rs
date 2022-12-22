mod common;
mod executor;

use executor::Executor;
use anyhow::Result;
use std::{
    collections::VecDeque,
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs},
    sync::{Arc, Mutex},
};

type Handler = Box<dyn Fn(Request) -> Response>;

struct Request {
    stream: TcpStream,
    buf: Vec<u8>,
}

struct Response {}

pub struct Server {
    receiver: Receiver,
    handler: Handler,
    executor: Executor,
}

pub struct Receiver {
    addr: SocketAddr,
    message_queue: Arc<Mutex<VecDeque<Request>>>,
}

impl Receiver {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Receiver> {
        Self::from_listener(TcpListener::bind(addr)?)
    }

    pub fn from_listener(listener: TcpListener) -> Result<Receiver> {
        let addr = listener.local_addr()?;

        let queue = Arc::new(Mutex::new(VecDeque::new()));
        let queue_inner = queue.clone();

        let _ = std::thread::spawn(move || {
            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                let mut buf = Vec::new();

                stream.read_to_end(&mut buf).unwrap();
                let req = Request { stream, buf };

                queue_inner.lock().unwrap().push_back(req);
            }
        });

        Ok(Receiver {
            addr,
            message_queue: queue,
        })
    }
    
    pub fn run(&mut self) -> Result<()> {
        loop {
            self.message_queue.lock().unwrap();
            let next = self.message_queue
        }
        
        Ok(())
    }
}
