use crate::device::ConnectedDevice;
use std::str::FromStr;

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

    pub fn parse_performance(response: &str) {
        println!("{}", response);
    }

    pub fn parse_connected_devices(response: &str) -> Vec<ConnectedDevice> {
        println!("{:?}", response);
        response
            .lines()
            .map(|a| ConnectedDevice::from_str(&a).unwrap())
            .collect()
    }
}
