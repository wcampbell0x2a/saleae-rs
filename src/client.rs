//! This module defines the client data structure - the main entry point of communication
//! to the saleae

use anyhow::Result;
use std::io::prelude::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

use crate::device::ConnectedDevice;
use crate::response::Response;

#[derive(Debug)]
/// Main interface for communication to Saleae Logic
pub struct Client {
    /// tcp stream with connection to saleae
    stream: TcpStream,
}

/// Constructor
impl Client {

    /// constructor
    pub fn new(stream: TcpStream) -> Result<Client> {
        Ok(Client { stream: stream })
    }
}

/// Interface for setting and getting Logic information
impl Client {
    /// Return current performance level of Logic
    pub fn get_performance(&mut self) -> Result<u8> {
        self.run_command("get_performance\0")?;
        //TODO lol clean this up
        let r: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Response::verify_ack(&r);
        Ok(Response::parse_performance(&Response::remove_ack(&r)))
    }

    /// Return current connected devices of Logic
    pub fn get_connected_devices(&mut self) -> Result<Vec<ConnectedDevice>> {
        self.run_command("get_connected_devices\0")?;
        //TODO lol clean this up
        let r: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Response::verify_ack(&r);
        Ok(Response::parse_connected_devices(&Response::remove_ack(&r)))
    }
}

/// private functions for socket functions
impl Client {
    fn read_line(&mut self) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.stream);
        let mut buf = [0; 120];
        reader.read(&mut buf)?;
        Ok(buf.to_vec())
    }

    //TODO Support for parameters
    fn run_command(&mut self, command: &str) -> Result<()> {
        let mut writer = BufWriter::new(&self.stream);
        writer.write(command.as_bytes()).unwrap();
        Ok(())
    }
}
