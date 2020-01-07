#[cfg(test)]
mod tests {
    use saleae::device::{ConnectedDevice, DeviceID};
    use saleae::response::Response;
    use saleae::samplerate::SampleRate;

    #[test]
    fn test_remove_ack() {
        let test0 = String::from("let\nmy\npeople\ngo\nACK");
        assert_eq!("let\nmy\npeople\ngo", Response::remove_ack(&test0));

        let test1 = String::from("\nACK");
        assert_eq!("", Response::remove_ack(&test1));

        let test2 = String::from("let\nmy\npeople\ngo\nACK\u{0}\u{0}");
        assert_eq!("let\nmy\npeople\ngo", Response::remove_ack(&test2));
    }

    #[test]
    fn test_verify_ack() {
        let test0 = String::from("let\nmy\npeople\ngo\nACK");
        assert!(Response::verify_ack(&test0));

        let test1 = String::from("ACK");
        assert!(Response::verify_ack(&test1));

        let test2 = String::from("ack");
        assert_eq!(false, Response::verify_ack(&test2));

        let test3 = String::from("let\nmy\npeople\ngo\n");
        assert_eq!(false, Response::verify_ack(&test3));
    }

    #[test]
    fn test_parse_performance() {
        assert_eq!(Response::parse_performance("100"), 100);
        assert_eq!(Response::parse_performance("80"), 80);
        assert_eq!(Response::parse_performance("60"), 60);
        assert_eq!(Response::parse_performance("40"), 40);
        assert_eq!(Response::parse_performance("20"), 20);
    }

    #[test]
    fn test_get_all_sample_rates() {
        let one = SampleRate {
            AnalogSampleRate: 10000000,
            DigitalSampleRate: 625000,
        };
        let two = SampleRate {
            AnalogSampleRate: 5000000,
            DigitalSampleRate: 1250000,
        };
        let expected = vec![one, two];
        let result = Response::parse_get_all_sample_rates(
            "10000000, 625000
                                                           5000000, 1250000",
        );
        assert_eq!(result[0], expected[0]);
        assert_eq!(result[1], expected[1]);
    }

    #[test]
    fn test_connected_devices() {
        let one = ConnectedDevice {
            d_type: "2".to_string(),
            name: "Logic 8".to_string(),
            device_id: DeviceID::LOGIC_8_DEVICE,
            index: "0xffffffffff".to_string(),
            is_active: true,
        };
        let two = ConnectedDevice {
            d_type: "1".to_string(),
            name: "Logic Pro 16".to_string(),
            device_id: DeviceID::LOGIC_PRO_16_DEVICE,
            index: "0xdf03c43d1f3aa2f3".to_string(),
            is_active: false,
        };
        let expected = vec![one, two];
        let result = Response::parse_connected_devices(
            "2, Logic 8, LOGIC_8_DEVICE, 0xffffffffff, ACTIVE
             1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3",
        );
        assert_eq!(result[0], expected[0]);
        assert_eq!(result[1], expected[1]);
    }

    #[test]
    fn test_is_processing_complete() {
        assert_eq!(true, Response::parse_processing_complete("TRUE"));
        assert_eq!(false, Response::parse_processing_complete("FALSE"));
    }

    #[test]
    fn test_parse_num_samples() {
        assert_eq!(10000, Response::parse_num_samples("10000"));
        assert_eq!(20, Response::parse_num_samples("20"));
    }
}
