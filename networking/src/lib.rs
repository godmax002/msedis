use net2::{TcpBuilder, TcpStreamExt} ;

use database::Database;
use parser::Parser;
use parser::ParseError;

use std::net::ToSocketAddrs;
use std::sync::atomic::AtomicUsize;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::{net::TcpStream, os::unix::net::UnixStream, time::Duration};
use std::io::{self, Read, Write};

enum Stream {
    Tcp(TcpStream),
    Unix(UnixStream),
}

impl Stream {
    fn try_clone(&self) -> io::Result<Stream> {
        match self {
            Stream::Tcp(s) => Ok(Stream::Tcp(s.try_clone())),
            Stream::Unix(s) => Ok(Self::Unix(s.try_clone())),
        }
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Stream::Tcp(s) => s.write(buf),
            Self::Unix(s) => s.write(buf),
        }
    }

    fn set_keepalive(&self, duration: Option<Duration>) -> io::Result<()>{
        match self {
            Stream::Tcp(s) => TcpStreamExt.set_keepalive(s, duration),
            Stream::Unix(_) => Ok(()),
        }
    }

    fn set_write_timeout(&self, duration: Option<Duration>) -> io::Result<()>{
        match self {
            Stream::Tcp(s) => TcpStreamExt.set_write_timeout(s, duration),
            Stream::Unix(_) => Ok(()),
        }
    }

    fn set_read_timeout(&self, duration: Option<Duration>) -> io::Result<()>{
        match self {
            Stream::Tcp(s) => TcpStreamExt.set_read_timeout(s, duration),
            Stream::Unix(_) => Ok(()),
        }
    }
}

impl Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Stream::Tcp(s) => s.read(buf),
            Stream::Unix(s) => s.read(buf),
        }
    }
}

struct Client {
    stream: Stream,
    db: Arc<Mutex<Database>> ,
    id: usize,
}

pub struct Server {
    db: Arc<Mutex<Database>>,
    listener_channels: Vec<Sender<u8>>,
    listener_threads: Vec<thread::JoinHandle<()>>,
    pub next_id: Arc<AtomicUsize>,
    hz_stop: Option<Sender<()>>,
}

impl Client {
    pub fn tcp(stream: TcpStream, db: Arc<Mutex<Database>>, id: usize) -> Client {
        Client { stream: Stream::Tcp(stream), db: db, id: id }
    }
    
    pub fn unix(stream: UnixStream, db: Arc<Mutex<Database>>, id: usize) -> Client {
        Client { stream: Stream::Unix(steam), db: db, id: id }
    }

    // creates a thread that writes into the client stream each response received
    fn create_writer_thread(
        &self,
        sender: Sender<(Level, String)>,
        rx: Receiver<Option<Response>>,
    ) {
        let mut stream = self.stream.try_clone().unwrap();

        thread::spawn(move || {
            while let Ok(Some(msg)) = rx.recv() {
                match stream.write(msg.as_bytes) {
                    Ok(_) => (),
                    Err(e) => {
                        sendlog!(sender, Warning, "Error write to client : {:?}", e)
                    }
                };
            }
        })

    }

    // runs all clients commands. The function loops until the client disconnect
    // 1.read stream to buffer 2. parse 3. lock db and execute 4.response
    pub fn run(&mut self, sender: Sender<(Level, String)>) {
        let mut parser = Parser::new();

        loop {
            // 1. read stream to parser buffer
            parser.allocate();
            let len = {
                let pos = parser.written;
                let buffer = parser.get_mut();
                match self.stream.read(buffer) {
                    Ok(r) => r,
                    Err(e) => {
                        sendlog!(sender, Verbose, "Reading from client: {:?}", e);
                        break;
                    }
                };
            };
            parser.written += len;
            if len == 0 {
                sendlog!(sender, Verbose, "Client closed connection");
                break;
            }

            // 2. parse
            let parsed_command =  match parser.next() {
                Ok(p) => p,
                Err(err) => {
                    match err {
                        ParseError::Incomplete => continue,
                        ParseError::BadProtocol(s) => {

                        },
                        _ => {

                        },
                    }

                }
            };

            // 3. lock db and execute
            let r = {
                let mut db = match self.db.lock() {
                    Ok(db) => db,
                    Err(_) => break,
                };
                command::command(parsed_command, &mut *db, &mut client)
            };
        }

    }
}

impl Server {
    pub fn new(config: Config) -> Server {

    }

    pub fn get_mut_db(&self) -> MutexGuard<database::Database> {

    }

    pub fn run(&mut self) {

    }
    
    fn reuse_address(&self, builder: &TcpBuilder) -> io::Result<()> {
    
    }

    pub fn join(&mut self) {

    }

    fn listen<T: ToSocketAddrs> (
        &mut self,
        t: T,
        tcp_keepalive: u32,
        timeout: u64,
        tcp_backlog: i32
    ) -> io::Result<()>{

    }

    pub fn start(&mut self) {

    }

    pub fn stop(&mut self) {
    
    }

    
    
}