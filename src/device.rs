//! This module defines devices that are connected to saleae logic program
//!
//! ## Example of response from Logic
//! ```text
//! 1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3, ACTIVE
//! ```
//! ## C# struct
//! ```text, no_run
//! struct ConnectedDevices
//! {
//!   String type;
//!   String name;
//!   int device_id;
//!   int index;
//!   bool is_active;
//! }
//! ```

use anyhow::Result;
use std::str::FromStr;

custom_derive! {
    #[derive(Debug, EnumFromStr, PartialEq)]
    pub enum DeviceID {
        LOGIC_4_DEVICE,
        LOGIC_8_DEVICE,
        LOGIC_PRO_8_DEVICE,
        LOGIC_PRO_16_DEVICE,
    }
}

#[derive(Debug, PartialEq)]
pub struct ConnectedDevice {
    /// type of device
    pub d_type: String,
    /// name of device
    pub name: String,
    /// id of device
    pub device_id: DeviceID,
    /// index of device
    pub index: String,
    /// if device is active
    pub is_active: bool,
}

impl FromStr for ConnectedDevice {
    type Err = std::num::ParseIntError;
    fn from_str(response: &str) -> Result<Self, Self::Err> {
        let v: Vec<&str> = response.split(',').map(|a| a.trim_start()).collect();

        /* parse into device_id */
        let device_id: DeviceID = v[2].parse().unwrap();

        /*
         * last element is ACTIVE, if that element doesn't don't cause a panic by
         * checking that element
         */
        let mut is_active = false;
        if v.len() == 5 {
            if v[4] == "ACTIVE" {
                is_active = true;
            }
        }

        Ok(ConnectedDevice {
            d_type: v[0].to_string(),
            name: v[1].to_string(),
            device_id: device_id,
            index: v[3].to_string(),
            is_active: is_active,
        })
    }
}