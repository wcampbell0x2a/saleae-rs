//! This module help discern and parse the responses from saleae

use crate::device::ConnectedDevice;
use std::str::FromStr;

/// struct to handle responses
pub struct Response {}

//TODO add errors
impl Response {
    /// Return string without "\nACK" string line nor extra 0 char's from buffer
    pub fn remove_ack(response: &str) -> String {
        response
            .trim_end_matches(char::from(0))
            .trim_end_matches("\nACK")
            .to_string()
    }

    /// Check if last string is ACK
    pub fn verify_ack(response: &str) -> bool {
        response.lines().last().unwrap() == "ACK"
    }

    /// Parse the performance response
    ///
    /// # Sample Input
    /// The following values are expected as input:
    /// ```text, no_run
    /// 100
    /// 80
    /// 60
    /// 40
    /// 20
    /// ```
    //TODO finish this
    pub fn parse_performance(response: &str) -> u8 {
        response.parse::<u8>().unwrap()
    }

    /// Parse the connected_devices reponse into ConnectedDevice
    ///
    /// # Sample Input
    /// ```text
    /// 1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3, ACTIVE
    /// ```
    pub fn parse_connected_devices(response: &str) -> Vec<ConnectedDevice> {
        response
            .lines()
            .map(|a| ConnectedDevice::from_str(&a).unwrap())
            .collect()
    }
}
