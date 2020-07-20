//! This module helps create requests to the Saleae Logic software
//!
//! The function here parse and type check the input from the API into strings to send into
//! the Saleae socket
use anyhow::{anyhow, Result};

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

        let d_str = if !digital_channels.is_empty() {
            format!(
                ", digital_channels, {}",
                Self::create_channel_str(digital_channels)?
            )
        } else {
            "".to_string()
        };

        let a_str = if !analog_channels.is_empty() {
            format!(
                ", analog_channels, {}",
                Self::create_channel_str(analog_channels)?
            )
        } else {
            "".to_string()
        };

        Ok(format!("set_active_channels{}{}\0", d_str, a_str))
    }
}

/// Helper functions
impl Request {
    pub fn create_channel_str(v: &[u8]) -> Result<String> {
        let s = v
            .iter()
            .map(|a| format!("{}, ", a.to_string()))
            .collect::<String>();
        Ok(s[..s.len() - 2].to_string())
    }
}
