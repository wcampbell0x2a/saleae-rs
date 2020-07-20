use saleae::device::{ConnectedDevice, DeviceID};
use saleae::response;
use saleae::samplerate::SampleRate;

#[test]
fn test_remove_ack() {
    let test0 = String::from("let\nmy\npeople\ngo\nACK");
    assert_eq!("let\nmy\npeople\ngo", response::remove_ack(&test0));

    let test1 = String::from("\nACK");
    assert_eq!("", response::remove_ack(&test1));

    let test2 = String::from("let\nmy\npeople\ngo\nACK\u{0}\u{0}");
    assert_eq!("let\nmy\npeople\ngo", response::remove_ack(&test2));
}

#[test]
fn test_verify_ack() {
    let test0 = String::from("let\nmy\npeople\ngo\nACK");
    assert!(response::verify_ack(&test0));

    let test1 = String::from("ACK");
    assert!(response::verify_ack(&test1));

    let test2 = String::from("ack");
    assert_eq!(false, response::verify_ack(&test2));

    let test3 = String::from("let\nmy\npeople\ngo\n");
    assert_eq!(false, response::verify_ack(&test3));
}

#[test]
fn test_parse_performance() {
    assert_eq!(response::parse_performance("100"), 100);
    assert_eq!(response::parse_performance("80"), 80);
    assert_eq!(response::parse_performance("60"), 60);
    assert_eq!(response::parse_performance("40"), 40);
    assert_eq!(response::parse_performance("20"), 20);
}

#[test]
fn test_get_all_sample_rates() {
    let one = SampleRate {
        DigitalSampleRate: 25_000_000,
        AnalogSampleRate: 3_125_000,
    };
    let two = SampleRate {
        DigitalSampleRate: 6_250_000,
        AnalogSampleRate: 1_562_500,
    };
    let result = response::parse_get_all_sample_rates("25000000, 3125000\n6250000, 1562500\n");
    assert_eq!(result[0], one);
    assert_eq!(result[1], two);
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
    let result = response::parse_connected_devices(
        "2, Logic 8, LOGIC_8_DEVICE, 0xffffffffff, ACTIVE
         1, Logic Pro 16, LOGIC_PRO_16_DEVICE, 0xdf03c43d1f3aa2f3",
    );
    assert_eq!(result[0], expected[0]);
    assert_eq!(result[1], expected[1]);
}

#[test]
fn test_parse_into_active_channels() {
    // Test regular input
    let input: String = "digital_channels, 0, 4, 5, 7, analog_channels, 0, 1, 2, 5".to_string();
    let result = response::parse_get_active_channels(&input).unwrap();
    assert_eq!([0, 4, 5, 7], result[0].as_slice());
    assert_eq!([0, 1, 2, 5], result[1].as_slice());

    // Test with no analog channels
    let input: String = "digital_channels, 0, 4, 5, 7, analog_channels".to_string();
    let result = response::parse_get_active_channels(&input).unwrap();
    assert_eq!([0, 4, 5, 7], result[0].as_slice());
    assert!(result[1].as_slice().is_empty());

    // Test with no digital channels
    let input: String = "digital_channels, analog_channels, 1, 2, 5".to_string();
    let result = response::parse_get_active_channels(&input).unwrap();
    assert!(result[0].as_slice().is_empty());
    assert_eq!([1, 2, 5], result[1].as_slice());
}

#[test]
fn test_is_processing_complete() {
    assert_eq!(true, response::parse_processing_complete("TRUE"));
    assert_eq!(false, response::parse_processing_complete("FALSE"));
}

#[test]
fn test_parse_num_samples() {
    assert_eq!(10000, response::parse_num_samples("10000"));
    assert_eq!(20, response::parse_num_samples("20"));
}
