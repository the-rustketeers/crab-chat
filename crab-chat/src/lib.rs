use json::JsonValue;
use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub const ADDRESS: &str = "127.0.0.1:13579";

/**
 * Sends a json object through the TcpStream, by first creating the object, sending its size, and then the object.
 */
pub fn send_json_packet(s: &mut TcpStream, obj: JsonValue) {
    let strung = obj.dump();
    let pack_size = strung.len();

    s.write_all(&pack_size.to_be_bytes()).unwrap();
    s.write_all(strung.as_bytes()).unwrap();
}

/**
 * Receives a packet through the TcpStream, by first reading the size of the json packet, then reading the packet, then finally parsing the json packet.
 */
pub fn receive_json_packet(s: &mut TcpStream) -> JsonValue {
    let mut packet_size_buf: [u8; 8] = [0; 8];
    s.read_exact(&mut packet_size_buf).unwrap();

    let packet_size: usize = usize::from_be_bytes(packet_size_buf);

    let mut packet_buf: Vec<u8> = vec![0; usize::from(packet_size)];

    s.read_exact(&mut packet_buf).unwrap();
    let packet: &str = str::from_utf8(&packet_buf).unwrap();

    json::parse(packet).unwrap()
}


/* CODE BELOW IS FROM RUST DOCS!
I wanted to just toss in the direct thread management and implementation they added.
Seems to work, although may not be optimal. 
 */
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

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
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}
