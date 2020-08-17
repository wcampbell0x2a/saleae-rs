//! This module help discern and parse the responses from saleae

use crate::device::ConnectedDevice;
use crate::samplerate::SampleRate;
use anyhow::Result;
use std::str::FromStr;

//TODO add errors
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
pub fn parse_performance(response: &str) -> u8 {
    response.parse::<u8>().unwrap()
}

pub fn parse_num_samples(response: &str) -> u32 {
    response.parse::<u32>().unwrap()
}

pub fn parse_get_sample_rate(response: &str) -> SampleRate {
    let mut iter = response.lines();
    SampleRate {
        DigitalSampleRate: iter.next().unwrap().parse::<u32>().unwrap(),
        AnalogSampleRate: iter.next().unwrap().parse::<u32>().unwrap(),
    }
}

/// Parse get all sample rates
///
/// # Sample input
/// ```text
/// 5000000, 1250000
/// 10000000, 625000
/// ```
pub fn parse_get_all_sample_rates(response: &str) -> Vec<SampleRate> {
    response
        .lines()
        .map(|a| SampleRate::from_str(a).unwrap())
        .collect()
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
        .map(|a| ConnectedDevice::from_str(a).unwrap())
        .collect()
}

pub fn parse_get_active_channels(response: &str) -> Result<Vec<Vec<u8>>> {
    println!("{}", response);
    let v: Vec<&str> = response.split(',').map(str::trim_start).collect();

    // Find position of starter word
    let digital_pos = v.iter().position(|a| *a == "digital_channels").unwrap();
    let analog_pos = v.iter().position(|a| *a == "analog_channels").unwrap();

    // Parse in between words to find values
    let digital_res: Vec<u8> = v[digital_pos + 1..analog_pos]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect();

    let analog_res: Vec<u8> = v[analog_pos + 1..v.len()]
        .iter()
        .map(|a| a.parse().unwrap())
        .collect();
    Ok(vec![digital_res, analog_res])
}

/// Parse if processing is complete
///
/// # Sample Input
/// ```text
/// FALSE
/// ```
/// ```text
/// TRUE
/// ```
pub fn parse_processing_complete(response: &str) -> bool {
    response == "TRUE"
}
