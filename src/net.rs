use std::io::Error as IoError;
use std::net::{TcpListener as NetTcpListener, TcpStream as NetTcpStream, ToSocketAddrs};

use crate::ReadWrite;

impl ReadWrite for NetTcpStream {}

pub struct TcpStream<A: ToSocketAddrs> {
    addr: A,
}

impl<A: ToSocketAddrs> TcpStream<A> {
    pub fn new(addr: A) -> TcpStream<A> {
        TcpStream { addr }
    }
}

impl<A: ToSocketAddrs> Iterator for TcpStream<A> {
    type Item = Result<Box<dyn ReadWrite>, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        match NetTcpStream::connect(&self.addr) {
            Ok(stream) => Some(Ok(Box::new(stream))),
            Err(err) => Some(Err(err))
        }
    }
}

pub struct TcpListener {
    listener: NetTcpListener,
}

impl TcpListener {
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<TcpListener, IoError> {
        match NetTcpListener::bind(addr) {
            Ok(listener) => Ok(TcpListener { listener }),
            Err(err) => Err(err),
        }
    }
}

impl Iterator for TcpListener {
    type Item = Result<Box<dyn ReadWrite>, IoError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.listener.accept() {
            Ok((stream, _)) => Some(Ok(Box::new(stream))),
            Err(err) => Some(Err(err))
        }
    }
}