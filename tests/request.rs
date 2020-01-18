#[cfg(test)]
mod tests {
    use anyhow::Result;
    use saleae::request::Request;
    /// Regular function tests
    #[test]
    fn test_prepare_set_active_channels() {
        let expected1: String =
            "set_active_channels, digital_channels, 1, 7, 8, 9, analog_channels, 0, 9, 8\0"
                .to_string();
        assert_eq!(
            expected1,
            Request::prepare_set_active_channels(&[1, 7, 8, 9], &[0, 9, 8]).unwrap(),
        );

        // Empty digital channel
        let expected2: String = "set_active_channels, analog_channels, 0, 9, 8\0".to_string();
        assert_eq!(
            expected2,
            Request::prepare_set_active_channels(&[], &[0, 9, 8]).unwrap(),
        );

        // Empty analog channel
        let expected3: String = "set_active_channels, digital_channels, 1, 7, 8, 9\0".to_string();
        assert_eq!(
            expected3,
            Request::prepare_set_active_channels(&[1, 7, 8, 9], &[]).unwrap(),
        );
    }

    #[test]
    #[should_panic(
        expected = "Logic requires at least one active channel, no active channels found"
    )]
    fn test_prepare_set_active_channels_panic() {
        // Empty both channels (should panic / error)
        let expected4: String = "set_active_channels\0".to_string();
        assert_eq!(
            expected4,
            Request::prepare_set_active_channels(&[], &[]).unwrap(),
        );
    }

    #[test]
    fn test_prepare_export_data2() {
        // all with analog
        let expected: String =  "export_data2, /tmp/test.csv, ALL_CHANNELS, ALL_TIME, CSV, HEADERS, COMMA, HEX, VOLTAGE".to_string();
        assert_eq!(Request::prepare_export_data2(
            "/tmp/test.csv".to_string(),
            &vec![],
            &vec![],
            /* active_digital */ &[3, 4, 5],
            /* active_analog */ &[5, 6, 7],
        ).unwrap(), expected);

        // all without analog
        let expected1: String = "export_data2, /tmp/test.csv, ALL_CHANNELS, ALL_TIME, CSV, HEADERS, COMMA, TIME_STAMP, COMBINED, HEX, ROW_PER_CHANGE".to_string();
        assert_eq!(Request::prepare_export_data2(
            "/tmp/test.csv".to_string(),
            &vec![],
            &vec![],
            /*active_digital */ &[3, 4, 5],
            &vec![],
        ).unwrap(), expected1);
    }

    #[test]
    fn test_create_channel_str() {
        let input = [0, 4, 5, 7];
        assert_eq!("0, 4, 5, 7", Request::create_channel_str(&input).unwrap());

        let input = [1, 2, 3, 5, 8];
        assert_eq!(
            "1, 2, 3, 5, 8",
            Request::create_channel_str(&input).unwrap()
        );
    }
}
