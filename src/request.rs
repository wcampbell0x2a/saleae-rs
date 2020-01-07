//! This module helps create requests to the Saleae Logic software
//!
//! The function here parse the input from the API into strings to send into
//! the Saleae socket

pub struct Request {}

impl Request {
    pub fn create_channel_str(v: &[u8]) -> String
    {
        let s = v.into_iter().map(|a| format!("{}, ", a.to_string())).collect::<String>();
        s[..s.len() - 2].to_string()
    }
}
