//! This module helps create requests to the Saleae Logic software
//!
//! The function here parse and type check the input from the API into strings to send into
//! the Saleae socket
use anyhow::Result;

pub struct Request {}

impl Request {
     pub fn prepare_set_active_channels(digital_channels: &[u8], analog_channels: &[u8]) -> Result<String>
     {
        /* TODO verify channels being selected? */
        let d_str: String = format!("digital_channels, {}", Request::create_channel_str(digital_channels)?);
        let a_str: String = format!("analog_channels, {}", Request::create_channel_str(analog_channels)?);

        Ok(format!("set_active_channels, {}, {}\0", d_str, a_str))
     }
}

/// Helper functions
impl Request {
    pub fn create_channel_str(v: &[u8]) -> Result<String>
    {
        let s = v.into_iter().map(|a| format!("{}, ", a.to_string())).collect::<String>();
        Ok(s[..s.len() - 2].to_string())
    }
}
