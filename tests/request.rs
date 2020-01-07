#[cfg(test)]
mod tests {
    use saleae::request::Request;

    #[test]
    fn test_create_channel_str() {
        let input = [0, 4, 5, 7];
        assert_eq!("0, 4, 5, 7", Request::create_channel_str(&input));

        let input = [1, 2, 3, 5, 8];
        assert_eq!("1, 2, 3, 5, 8", Request::create_channel_str(&input));
    }
}
