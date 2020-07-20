//! This module defines devices that are connected to the saleae logic program.
//!
//! ## Example of response from Logic
//! ```text
//! 1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3, ACTIVE
//! ```
//! ## C# struct
//! ```text
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

// TODO Look for better EnumFromStr derive
custom_derive! {
    /// Device id for saleae devices
    #[allow(non_camel_case_types)]
    #[derive(Debug, EnumFromStr, PartialEq)]
    pub enum DeviceID {
        /// Regular 4 Wire device
        LOGIC_4_DEVICE,
        /// Regular 8 Wire device
        LOGIC_8_DEVICE,
        /// Pro 8 Wire device
        LOGIC_PRO_8_DEVICE,
        /// Pro 16 Wire device
        LOGIC_PRO_16_DEVICE,
    }
}

/// Connected Device to saleae, the main usability comes from the fromStr trait.
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
        let v: Vec<&str> = response.split(',').map(str::trim_start).collect();

        /* parse into device_id */
        let device_id: DeviceID = v[2].parse().unwrap();

        /*
         * last element is ACTIVE, if that element doesn't don't cause a panic by
         * checking that element
         */
        let is_active = v.len() == 5 && v[4] == "ACTIVE";

        Ok(Self {
            d_type: v[0].to_string(),
            name: v[1].to_string(),
            device_id,
            index: v[3].to_string(),
            is_active,
        })
    }
}
