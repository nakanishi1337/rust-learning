#[cfg(test)]
mod tests {

    #[test]
    fn catch_error() {
        let result: Result<i32, &str> = Err("an error occurred");
        let value = match result {
            Ok(v) => v,
            Err(e) => {
                println!("Caught an error: {}", e);
                0 // デフォルト値を返す
            }
        };
        assert_eq!(value, 0);
    }
}
