//! This module defines the client data structure - the main entry point of communication
//! to the saleae

use anyhow::Result;
use std::io::prelude::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

use crate::device::ConnectedDevice;
use crate::performance::PerformanceOption;
use crate::request::Request;
use crate::response::Response;
use crate::samplerate::SampleRate;

#[faux::create]
#[derive(Debug)]
pub struct Connection {
    stream: TcpStream,
}

#[faux::methods]
impl Connection {
    pub fn new(ip_port: &str) -> Self {
        Connection {
            stream: TcpStream::connect(ip_port).unwrap(),
        }
    }

    pub fn general_recieve_ack(&mut self) -> Result<bool> {
        let r: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Ok(Response::verify_ack(&r))
    }

    pub fn general_recieve_message(&mut self) -> Result<String> {
        let msg: String = std::str::from_utf8(&self.read_line()?)?.to_string();
        Response::verify_ack(&msg);
        Ok(msg)
    }

    fn read_line(&mut self) -> Result<Vec<u8>> {
        let mut reader = BufReader::new(&self.stream);
        let mut buf = [0; 500];
        let len = reader.read(&mut buf)?;
        if len < 1 {
            panic!("read buffer len < 0");
        }
        Ok(buf[..len].to_vec())
    }

    //TODO Support for parameters
    pub fn run_command(&mut self, command: &str) -> Result<()> {
        let mut writer = BufWriter::new(&self.stream);
        let len = writer.write(command.as_bytes()).unwrap();
        if len < 1 {
            panic!("write buffer len < 0");
        }
        Ok(())
    }
}

#[derive(Debug)]
/// Main interface for communication to Saleae Logic
pub struct Client {
    /// tcp stream with connection to saleae
    connection: Connection,
}

/// Constructor
impl Client {
    /// constructor
    //TODO make this create a connection from a string
    pub fn new(connection: Connection) -> Result<Client> {
        Ok(Client { connection })
    }
}

/// Interface for setting and getting Logic information
impl Client {
    /// Set a trigger of channels
    /// TODO create trigger methods/structs/tests

    pub fn set_num_samples(&mut self, num: u32) -> Result<bool> {
        self.connection
            .run_command(&format!("set_num_samples, {}\0", num))?;
        Ok(self.connection.general_recieve_ack()?)
    }

    pub fn get_num_samples(&mut self) -> Result<u32> {
        self.connection.run_command("get_num_samples\0")?;
        let response = self.connection.general_recieve_message()?;
        if Response::verify_ack(&response) {
            Ok(Response::parse_num_samples(&Response::remove_ack(
                &response,
            )))
        } else {
            Err(anyhow!("No ACK found"))
        }
    }

    /// Set the capture duration to a length of time
    pub fn set_capture_seconds(&mut self, seconds: f32) -> Result<bool> {
        self.connection
            .run_command(&format!("set_capture_seconds, {}\0", seconds))?;
        Ok(self.connection.general_recieve_ack()?)
    }

    /// Set sample rate of saleae
    ///
    /// Note: Make sure to run `get_all_sample_rates` and set it from a available
    /// sample rate
    pub fn set_sample_rate(&mut self, rate: &SampleRate) -> Result<bool> {
        self.connection.run_command(&format!(
            "set_sample_rate, {}, {}\0",
            rate.DigitalSampleRate, rate.AnalogSampleRate
        ))?;
        Ok(self.connection.general_recieve_ack()?)
    }

    pub fn get_sample_rate(&mut self) -> Result<SampleRate> {
        self.connection.run_command("get_sample_rate\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_get_sample_rate(&Response::remove_ack(
            &response,
        )))
    }

    pub fn get_all_sample_rates(&mut self) -> Result<Vec<SampleRate>> {
        self.connection.run_command("get_all_sample_rates\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_get_all_sample_rates(&Response::remove_ack(
            &response,
        )))
    }

    /// Return current performance level of Logic
    pub fn get_performance(&mut self) -> Result<u8> {
        self.connection.run_command("get_performance\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_performance(&Response::remove_ack(
            &response,
        )))
    }

    /// Set the performance value, controlling the USB traffic and quality
    pub fn set_performance(&mut self, perf: PerformanceOption) -> Result<bool> {
        let input = String::from(&format!("set_performance, {}\0", perf as i32));
        self.connection.run_command(&input)?;
        Ok(self.connection.general_recieve_ack()?)
    }

    //TODO get_capture_pretrigger_buffer_size

    /// Return current connected devices of Logic
    pub fn get_connected_devices(&mut self) -> Result<Vec<ConnectedDevice>> {
        self.connection.run_command("get_connected_devices\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_connected_devices(&Response::remove_ack(
            &response,
        )))
    }

    /// Find index of device from the list of devices connected to Saleae
    ///
    /// Note: Indices start at 1, not 0
    /// TODO: test with multiple saleae
    pub fn select_active_device(&mut self, device: ConnectedDevice) -> Result<bool> {
        let b = self
            .get_connected_devices()
            .unwrap()
            .into_iter()
            .position(|a| a == device);
        self.connection
            .run_command(&format!("select_active_device, {}", b.unwrap() + 1))?;
        //TODO check ack?
        Ok(true)
    }

    /// Return current active device of Logic
    pub fn get_active_device(&mut self) -> Result<ConnectedDevice> {
        self.connection.run_command("get_connected_devices\0")?;
        //TODO lol clean this up
        let response = self.connection.general_recieve_message()?;
        Ok(
            Response::parse_connected_devices(&Response::remove_ack(&response))
                .into_iter()
                .find(|a| a.is_active)
                .unwrap(),
        )
    }

    /// Parse the get active channels command into tuples of digital and analog
    /// channels that are current
    pub fn get_active_channels(&mut self) -> Result<(Vec<Vec<u8>>)> {
        self.connection.run_command("get_active_channels\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_get_active_channels(&Response::remove_ack(
            &response,
        ))?)
    }

    /// Set the active channels for the Logic program
    ///
    /// # Example
    /// TODO add get_active_channels
    pub fn set_active_channels(
        &mut self,
        digital_channels: &[u8],
        analog_channels: &[u8],
    ) -> Result<bool> {
        self.connection
            .run_command(&Request::prepare_set_active_channels(
                &digital_channels,
                &analog_channels,
            )?)?;
        Ok(self.connection.general_recieve_ack()?)
    }

    /// Reset Active Channel
    pub fn reset_active_channels(&mut self) -> Result<bool> {
        self.connection.run_command("reset_active_channels\0")?;
        Ok(self.connection.general_recieve_ack()?)
    }

    //TODO get_digital_voltage_options OR get_full_scale_voltage_range
    //TODO set_full_scale_voltage_range

    /// Start Capture, without wating for ack/nack
    pub fn start_capture(&mut self) -> Result<bool> {
        self.connection.run_command("capture\0")?;
        //TODO check ack?
        Ok(true)
    }

    /// Start Capture, then wait until ack
    pub fn start_capture_block_until_finished(&mut self) -> Result<bool> {
        self.start_capture()?;
        Ok(self.connection.general_recieve_ack()?)
    }

    /// Check if processing is complete
    pub fn is_processing_complete(&mut self) -> Result<bool> {
        self.connection.run_command("is_processing_complete\0")?;
        let response = self.connection.general_recieve_message()?;
        Ok(Response::parse_processing_complete(&Response::remove_ack(
            &response,
        )))
    }

    /// Stop the saleae capture
    pub fn stop_capture(&mut self) -> Result<bool> {
        self.connection.run_command("stop_capture\0")?;
        Ok(self.connection.general_recieve_ack()?)
    }

    //TODO capture_to_file
    //TODO save_to_file
    //TODO load_from_file

    /// Close all tabs
    pub fn close_all_tabs(&mut self) -> Result<bool> {
        self.connection.run_command("close_all_tabs\0")?;
        Ok(self.connection.general_recieve_ack()?)
    }

    //TODO export_data2
    //TODO get_analyzers
    //TODO export_analyzer
    //TODO is_analyzer_complete
    //TODO get_capture_range
    //TODO get_viewstate
    //TODO get_viewstate
    //TODO exit
}
