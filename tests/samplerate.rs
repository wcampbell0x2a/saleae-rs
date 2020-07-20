use saleae::samplerate::SampleRate;
use std::str::FromStr;

#[test]
fn test_from_str() {
    assert_eq!(
        SampleRate::from_str("5000000, 1250000").unwrap(),
        SampleRate {
            DigitalSampleRate: 5_000_000,
            AnalogSampleRate: 1_250_000
        }
    );
    assert_eq!(
        SampleRate::from_str("10000000, 625000").unwrap(),
        SampleRate {
            DigitalSampleRate: 10_000_000,
            AnalogSampleRate: 625_000
        }
    );
}
