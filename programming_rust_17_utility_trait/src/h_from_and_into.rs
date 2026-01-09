#[cfg(test)]
mod tests {
    use std::num::ParseIntError;

    #[derive(Debug)]
    struct MyError;

    impl From<ParseIntError> for MyError {
        fn from(_: ParseIntError) -> Self {
            MyError
        }
    }

    fn parse_number(s: &str) -> Result<i32, MyError> {
        let n: i32 = s.parse()?; // ParseIntError → MyError に自動変換
        Ok(n)
    }

    #[test]
    fn parse_success() {
        let result = parse_number("42");
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn parse_failure_is_myerror() {
        let result = parse_number("abc");
        assert!(result.is_err());
    }
}
