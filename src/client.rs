//! This module defines the client data structure - the main entry point of communication
//! to the saleae

use anyhow::Result;
use bufstream::BufStream;
use std::io::prelude::{Read, Write};
use std::io::BufRead;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Client<S = TcpStream>
where
    S: Read + Write,
{
    socket: BufStream<S>,
}

impl Default for Client<TcpStream> {
    /// Connect to saleae on localhost with default port
    fn default() -> Client<TcpStream> {
        Client::<TcpStream>::connect("127.0.0.1:10429").unwrap()
    }
}

impl<S: Read + Write> Client<S> {
    pub fn connect<A: ToSocketAddrs>(addr: A) -> Result<Client<TcpStream>> {
        let stream = TcpStream::connect(addr).unwrap();
        Client::<TcpStream>::new(stream)
    }

    pub fn new(socket: S) -> Result<Client<S>> {
        let socket = BufStream::new(socket);

        //TODO Do some sort of checking if connected if that exists
        //
        Ok(Client { socket: socket })
    }

    pub fn get_performance(&mut self) -> Result<(String)> {
        self.run_command("GET_PERFORMANCE\0")?;
        self.read_line()
    }

    pub fn get_connected_devices(&mut self) -> Result<(String)> {
        self.run_command("GET_CONNECTED_DEVICES\0")?;
        self.read_line()
    }
}

impl<S: Read + Write> Client<S> {
    fn read_line(&mut self) -> Result<String> {
        println!("reading");
        let mut buf = String::new();
        self.socket.read_line(&mut buf)?;
        if buf.ends_with('\n') {
            buf.pop();
        }
        Ok(buf)
    }

    //TODO Support for parameters
    fn run_command(&mut self, command: &str) -> Result<()> {
        self.socket.write_all(command.as_bytes()).unwrap();
        Ok(())
    }
}
