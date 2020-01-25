use saleae;

use saleae::PerformanceOption;
use saleae::{Client, Connection};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut conn = Client::new(Connection::new("127.0.0.1:10429")).unwrap();
    let response0 = conn.get_performance();
    println!("get_performance: {}", response0.unwrap());

    //TODO for some reason this doesn't work/ change performance
    let response9 = conn.set_performance(PerformanceOption::Half);
    println!("set_performance: {:?}", response9.unwrap());

    let response1 = conn.get_connected_devices();
    println!("get_command_devices: {:#?}", response1.unwrap());

    let response2 = conn.get_active_device().unwrap();
    println!("active_device: {:#?}", response2);

    let response02 = conn.select_active_device(response2);
    println!("select_active_device: {:?}", response02.unwrap());

    let response3 = conn.reset_active_channels();
    println!("reset: {:?}", response3.unwrap());

    let response6 = conn.start_capture_block_until_finished();
    println!("longer waiting is complete: {:?}", response6.unwrap());

    let response = conn.set_capture_seconds(10.1);
    println!("set_capture_seconds: {:?}", response.unwrap());

    let response4 = conn.start_capture();
    println!("start_capture: {:?}", response4.unwrap());

    let response5 = conn.is_processing_complete();
    println!("is_processing_complete: {:?}", response5.unwrap());

    let response6 = conn.stop_capture();
    println!("stop_capture: {:?}", response6.unwrap());

    let duration = Duration::from_secs(2);
    thread::sleep(duration);

    let response7 = conn.is_processing_complete();
    println!("is_processing_complete: {:?}", response7.unwrap());

    let response8 = conn.close_all_tabs();
    println!("close_all_tabs: {:?}", response8.unwrap());

    let response10 = conn.set_num_samples(10000);
    println!("set_num_samples: {:?}", response10.unwrap());

    let response11 = conn.get_num_samples();
    println!("get_num_samples: {:?}", response11.unwrap());

    let response12 = conn.get_sample_rate();
    println!("get_sample_rate: {:?}", response12.unwrap());

    let response13 = conn.get_all_sample_rates().unwrap();
    println!("get_all_sample_rates: {:#?}", &response13);

    let sample_rate = &response13[1];

    let response14 = conn.set_sample_rate(&sample_rate);
    println!("set_sample_rate: {:?}", response14.unwrap());

    let response15 = conn.get_sample_rate();
    println!("get_sample_rate: {:?}", response15.unwrap());

    let response16 = conn.set_active_channels(&[0, 4, 5, 7], &[0, 1, 2, 5, 8]);
    println!("set_active_channels: {:?}", response16.unwrap());

    let response17 = conn.get_active_channels().unwrap();
    println!("digital: {:?}, analog: {:?}", response17[0], response17[1]);
}
