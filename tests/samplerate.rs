#[cfg(test)]
mod tests {
    use saleae::samplerate::SampleRate;
    use std::str::FromStr;

    #[test]
    fn test_from_str() {
        assert_eq!(
            SampleRate::from_str(&"5000000, 1250000").unwrap(),
            SampleRate {
                AnalogSampleRate: 5000000,
                DigitalSampleRate: 1250000
            }
        );
        assert_eq!(
            SampleRate::from_str(&"10000000, 625000").unwrap(),
            SampleRate {
                AnalogSampleRate: 10000000,
                DigitalSampleRate: 625000
            }
        );
    }
}
