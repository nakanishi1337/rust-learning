fn get_number() -> i32 {
    42
}

fn get_string() -> String {
    "Hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reference_to_return_value() {
        // 関数の戻り値を直接&で受け取って無名変数を作る
        let ref_num = &get_number();
        assert_eq!(*ref_num, 42);

        let ref_str = &get_string();
        assert_eq!(ref_str, &"Hello"); // &"Hello"もHelloを保存する無名変数への参照
    }
}
