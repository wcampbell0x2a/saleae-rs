//! this module emulates the following program:
//! [python-saleae-cli](https://github.com/saleae/python-saleae-cli)
//! the only change is this example has 10 captures 1 minutes each.
//!
use anyhow::Result;
use saleae::Client;
use saleae::PerformanceOption;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() -> Result<()> {
    let capture_count = 10;
    let capture_seconds = 10;

    let mut conn = Client::new(TcpStream::connect("127.0.0.1:10429").unwrap()).unwrap();

    for n in 0..capture_count {
        conn.set_capture_seconds(capture_seconds)?;
        println!("set capture seconds to: {}", capture_seconds);

        // capture, save to disk
        let filepath = format!("/home/wcampbell/logic/{}.logicdata", n).to_string();
        conn.capture_to_file(filepath)?;

        //        // csv raw output
        //        let csv_path = format!("~/logic/{}.csv", n).to_string();
        //        conn.export_data2(csv_path)?;
        //
        //        // analyzer export
        //        let analyzers = conn.get_analyzers()?;
        //        //TODO check if analyzers empty
        //        for analyzer in analyzers {
        //            let file_name = format!("{}_{}.csv", n, analyzer[0]);
        //            conn.export_analyzer(analyzer[1], file_path)?;
        //        }
    }

    Ok(())
}
