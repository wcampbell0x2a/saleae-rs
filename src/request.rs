//! This module helps create requests to the Saleae Logic software
//!
//! The function here parse and type check the input from the API into strings to send into
//! the Saleae socket
use anyhow::{Error, Result};

pub struct Request {}

impl Request {
    pub fn prepare_set_active_channels(
        digital_channels: &[u8],
        analog_channels: &[u8],
    ) -> Result<String> {
        // Check only one kind of empty
        if digital_channels.is_empty() && analog_channels.is_empty() {
            return Err(anyhow!(
                "Logic requires at least one active channel, no active channels found"
            ));
        }

        let mut d_str: String = "".to_string();
        if !digital_channels.is_empty() {
            d_str = format!(
                ", digital_channels, {}",
                Request::create_channel_str(digital_channels)?
            );
        }
        let mut a_str: String = "".to_string();
        if !analog_channels.is_empty() {
            a_str = format!(
                ", analog_channels, {}",
                Request::create_channel_str(analog_channels)?
            );
        }

        Ok(format!("set_active_channels{}{}\0", d_str, a_str))
    }

    pub fn prepare_export_data2(
        filepath: String,
        digital_channels: &[u8],
        analog_channels: &[u8],
        active_digital: &[u8],
        active_analog: &[u8],
    ) -> Result<String> {
        let input: String = format!("export_data2, {}, ", filepath);

        // channel selection
        let mut channel_type: String = "".to_string();
        let mut a_or_d: String = "".to_string();
        let mut channels: String = "".to_string();

        if digital_channels.is_empty() && analog_channels.is_empty() {
            channel_type = "ALL_CHANNELS, ".to_string();
        } else {
            if !active_digital.is_empty() && !active_analog.is_empty() {
                channel_type = "SPECIFIC_CHANNELS, ".to_string();
                if !digital_channels.is_empty() && !analog_channels.is_empty() {
                    a_or_d = "ANALOG_AND_DIGITAL, ".to_string();
                } else if !digital_channels.is_empty() {
                    a_or_d = "DIGITAL_ONLY, ".to_string();
                } else if !analog_channels.is_empty() {
                    a_or_d = "ANALOG_ONLY, ".to_string();
                }
            }

            // Add channels
            if !digital_channels.is_empty() {
                digital_channels
                    .iter()
                    .for_each(|&a| channels.push_str(&format!("{:?} DIGITAL", a)));
            }
            if !analog_channels.is_empty() {
                analog_channels
                    .iter()
                    .for_each(|&a| channels.push_str(&format!("{:?} ANALOG", a)));
            }
        }

        // Time
        // TODO support time entry
        let time: String = "ALL_TIME, ".to_string();

        // TODO support different export
        let mut csv = String::from("");
        if !active_analog.is_empty() {
            csv = "CSV, HEADERS, COMMA, HEX, VOLTAGE".to_string();
        }
        else {
            csv = "CSV, HEADERS, COMMA, TIME_STAMP, COMBINED, HEX, ROW_PER_CHANGE".to_string();
        }
        Ok(format!("{}{}{}{}{}", input, channel_type, a_or_d, time, csv))
    }
}

/// Helper functions
impl Request {
    pub fn create_channel_str(v: &[u8]) -> Result<String> {
        let s = v
            .into_iter()
            .map(|a| format!("{}, ", a.to_string()))
            .collect::<String>();
        Ok(s[..s.len() - 2].to_string())
    }
}
