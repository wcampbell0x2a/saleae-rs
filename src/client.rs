//! This module defines the client data structure - the main entry point of communication
//! to the saleae

use anyhow::Result;
use std::io::prelude::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn new(stream: TcpStream) -> Result<Client> {
        Ok(Client { stream: stream })
    }

    pub fn get_performance(&mut self) -> Result<(String)> {
        self.run_command("get_performance\0")?;
        //TODO lol clean this up
        let r: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Ok(r)
    }

    pub fn get_connected_devices(&mut self) -> Result<(String)> {
        self.run_command("get_connected_devices\0")?;
        //TODO lol clean this up
        let r: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Ok(r)
    }
}

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
