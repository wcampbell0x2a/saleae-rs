//! Saleae client for Rust
//!
//! This crate provides a Rust API for [Saleae](https://www.saleae.com).
//! The API is based on the documentation provided:
//! [SaleaeSocketAPI](https://github.com/saleae/SaleaeSocketApi)
//!
//! The main entry point for this API is the ['Client'](client/struct.Client.html) struct.
//!
//!
//! # Usage
//! ```text
//! [dependencies]
//! saleae = "*"
//! ```
//!
//! # Example with no error handling
//! ```rust, no_run
//! extern crate saleae;
//!
//! use saleae::Client;
//! use std::net::TcpStream;
//!
//! let mut conn = Client::new(TcpStream::connect("127.0.0.1:10429").unwrap()).unwrap();
//! let response0 = conn.get_performance();
//! println!("get_performance: {}", response0.unwrap());
//!
//! let response1 = conn.get_connected_devices();
//! println!("get_command_devices: {:?}", response1.unwrap());

#[macro_use]
extern crate anyhow;
extern crate bufstream;
#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

pub mod client;
pub mod device;
pub mod performance;
pub mod response;
pub mod samplerate;
pub mod request;

pub use client::Client;
pub use device::ConnectedDevice;
pub use performance::PerformanceOption;
pub use samplerate::SampleRate;
