#[cfg(test)]
mod tests {
    use saleae::client::{Client, Connection};
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
        let response = conn.set_sample_rate(&SampleRate {AnalogSampleRate: 6250000, DigitalSampleRate: 1562500}).unwrap();
        assert_eq!(true, response);
    }

    #[test]
    fn get_sample_rate() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("1000000\n0\n".to_string())) }

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
}
