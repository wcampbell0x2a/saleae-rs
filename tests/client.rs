#[cfg(test)]
mod tests {
    use saleae::client::{Client, Connection};

    #[test]
    fn test_client() {
        let mut conn = Connection::faux();
        unsafe { faux::when!(conn.run_command).then(|_| Ok(())) }
        unsafe { faux::when!(conn.general_recieve_message).then(|_| Ok("100".to_string())) }

        let mut conn = Client::new(conn).unwrap();
        let response = conn.get_performance().unwrap();
        assert_eq!(response, 100);
    }
}
