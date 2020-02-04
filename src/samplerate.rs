//! This module defines the sample rates returned and used by the saleae Logic program
//!
//! # C# Example
//! ```text
//! struct SampleRate
//! {
//!     public int AnalogSampleRate;
//!     public int DigitalSampleRate;
//! }
//! ```
use std::str::FromStr;

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct SampleRate {
    pub DigitalSampleRate: u32,
    pub AnalogSampleRate: u32,
}

impl FromStr for SampleRate {
    type Err = std::num::ParseIntError;
    fn from_str(response: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = response.split(',').map(|a| a.trim_start()).collect();

        Ok(SampleRate {
            DigitalSampleRate: v[0].parse::<u32>().unwrap(),
            AnalogSampleRate: v[1].parse::<u32>().unwrap(),
        })
    }
}
