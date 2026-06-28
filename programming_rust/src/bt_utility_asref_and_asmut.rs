/// AsRef と AsMut トレイトの学習
///
/// AsRef: 型を他の型への参照に変換
/// AsMut: 型を他の型への可変参照に変換

// String → &str, Vec<T> → &[T] などが組み込み
// AsRef を使う一般的な関数
fn print_ref<T: AsRef<str>>(value: T) {
    println!("📖 {}", value.as_ref());
}

// AsMut を使う一般的な関数
fn clear_vec<T: AsMut<[i32]>>(mut value: T) -> T {
    let slice = value.as_mut();
    for item in slice.iter_mut() {
        *item = 0;
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asref() {
        let s = String::from("Hello");
        print_ref(s); // String → &str
        print_ref("world"); // &str → &str
    }

    #[test]
    fn test_asmut() {
        let mut vec = vec![1, 2, 3];
        let result = clear_vec(&mut vec);

        assert_eq!(result, &[0, 0, 0]);
        println!("✅ 変更: {:?}", result);
    }
}
