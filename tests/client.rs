#[cfg(test)]
mod tests {
    use saleae::client::{Client, Connection};
    use saleae::device::DeviceID;
    use saleae::ConnectedDevice;
    use saleae::PerformanceOption;
    use saleae::SampleRate;

    #[test]
    fn set_num_samples() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.set_num_samples(500).unwrap();
        assert_eq!(true, response);
    }

    #[test]
    fn get_num_samples() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("3000\nACK".to_string())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_num_samples().unwrap();
        assert_eq!(3000, response);
    }

    #[test]
    #[should_panic]
    fn get_num_samples_fail() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("3000".to_string())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_num_samples().unwrap();
        assert_eq!(3000, response);
    }

    #[test]
    fn set_capture_seconds() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.set_capture_seconds(2.2).unwrap();
        assert_eq!(true, response);
    }

    #[test]
    fn set_sample_rate() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn
            .set_sample_rate(&SampleRate {
                AnalogSampleRate: 6250000,
                DigitalSampleRate: 1562500,
            })
            .unwrap();
        assert_eq!(true, response);
    }

    #[test]
    fn get_sample_rate() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe {
            faux::when!(conn.general_recieve_message).then(|_| Ok("1000000\n0\n".to_string()))
        }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_sample_rate().unwrap();
        assert_eq!(response.DigitalSampleRate, 1000000);
        assert_eq!(response.AnalogSampleRate, 0);
    }

    #[test]
    fn get_performance() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("100".to_string())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_performance().unwrap();
        assert_eq!(response, 100);
    }

    #[test]
    fn set_performance() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.set_performance(PerformanceOption::Full).unwrap();
        assert_eq!(response, true);

        let mut conn2 = Connection::faux();
        unsafe { faux::when!(conn2.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn2.general_recieve_ack).then(|_| Ok(false)) }

        let mut conn2 = Client::new(conn2).unwrap();
        let response2 = conn2.set_performance(PerformanceOption::Low).unwrap();
        assert_eq!(response2, false);
    }

    #[test]
    fn get_connected_devices() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe {
            faux::when!(conn.general_recieve_message).then(|_| Ok("1, Logic 8, LOGIC_8_DEVICE, 0x2dc9, ACTIVE\n2, Logic Pro 8, LOGIC_PRO_8_DEVICE, 0x7243\n3, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0x673f\n4, Logic 4, LOGIC_4_DEVICE, 0x6709\nACK".to_string()))
        }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_connected_devices().unwrap();
        assert_eq!(
            response[0],
            ConnectedDevice {
                d_type: "1".to_string(),
                name: "Logic 8".to_string(),
                device_id: DeviceID::LOGIC_8_DEVICE,
                index: "0x2dc9".to_string(),
                is_active: true
            }
        );
        assert_eq!(
            response[1],
            ConnectedDevice {
                d_type: "2".to_string(),
                name: "Logic Pro 8".to_string(),
                device_id: DeviceID::LOGIC_PRO_8_DEVICE,
                index: "0x7243".to_string(),
                is_active: false
            }
        );
        assert_eq!(
            response[2],
            ConnectedDevice {
                d_type: "3".to_string(),
                name: "Logic Pro 16".to_string(),
                device_id: DeviceID::LOGIC_PRO_16_DEVICE,
                index: "0x673f".to_string(),
                is_active: false
            }
        );
        assert_eq!(
            response[3],
            ConnectedDevice {
                d_type: "4".to_string(),
                name: "Logic 4".to_string(),
                device_id: DeviceID::LOGIC_4_DEVICE,
                index: "0x6709".to_string(),
                is_active: false
            }
        );
    }

    #[test]
    fn select_active_device() {
        //TODO can't test b/c of get_connected_devices not being fauxable
        //let device = ConnectedDevice {
        //    d_type: "4".to_string(),
        //    name: "Logic 4".to_string(),
        //    device_id: DeviceID::LOGIC_4_DEVICE,
        //    index: "0x6709".to_string(),
        //    is_active: false,
        //};

        //let mut conn = Connection::faux();
        //unsafe { faux::when!(conn.get_connected_devices).then(|_| Ok(device)) };
        //unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }

        //let mut conn = Client::new(conn).unwrap();
        //let response = conn.select_active_device(device).unwrap();
        //assert_eq!(response, true);
    }

    #[test]
    fn get_active_device() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe {
            faux::when!(conn.general_recieve_message).then(|_| {
                Ok("digital_channels, 0, 4, 5, 7, analog_channels, 0, 1, 2, 5, 8".to_string())
            })
        }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_active_channels().unwrap();
        assert_eq!(response[0], [0, 4, 5, 7]);
        assert_eq!(response[1], [0, 1, 2, 5, 8]);
    }

    #[test]
    fn set_active_channels() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn
            .set_active_channels(&[0, 4, 5, 7], &[0, 1, 2, 5, 8])
            .unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn reset_active_channels() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.reset_active_channels().unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn start_capture() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.start_capture().unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn start_capture_block_until_finished() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.start_capture_block_until_finished().unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn is_processing_complete() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("TRUE\nACK".to_string())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.is_processing_complete().unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn stop_capture() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.stop_capture().unwrap();
        assert_eq!(response, true);
    }

    #[test]
    fn close_all_tabs() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_ack).then(|_| Ok(true)) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.close_all_tabs().unwrap();
        assert_eq!(response, true);
    }
}
