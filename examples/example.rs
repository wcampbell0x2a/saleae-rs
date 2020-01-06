use saleae;

use saleae::Client;
use std::net::TcpStream;

fn main() {
    //let mut conn = Client::connect("127.0.0.1:6600").unwrap();
    let mut conn = Client::new(TcpStream::connect("127.0.0.1:10429").unwrap()).unwrap();
    let response0 = conn.get_performance();
    println!("get_performance: {}", response0.unwrap());

    let response1 = conn.get_connected_devices();
    println!("get_command_devices: {:?}", response1.unwrap());
}
