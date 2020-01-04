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
//! extern crate mpd;
//!
//! use mpd::Client;
//! use std::net::TcpStream;
//!
//! # fn main() {
//! let mut conn = Client::connect(TcpStream::connect("127.0.0.1:10429").unwrap()).unwrap();
//! conn.set_performance(Options::Performance::Full)
//! //let devices = conn.get_connected_devices()
//! //for device in devices {
//! //    println!(
//! #}

extern crate anyhow;
extern crate bufstream;

pub mod client;
pub use client::Client;
