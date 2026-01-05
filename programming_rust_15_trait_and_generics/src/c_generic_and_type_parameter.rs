// 複数の型パラメータを持つジェネリック関数

use std::fmt::Debug;

// 2つの異なる型のペアを作る
fn make_pair<T, U>(first: T, second: U) -> (T, U) {
    (first, second)
}

// 複数のトレイト境界: Clone + Debug + PartialOrd
fn print_and_clone_max<T: Clone + Debug + PartialOrd>(a: &T, b: &T) -> T {
    let max = if a > b { a } else { b };
    println!("Max value: {:?}", max);
    max.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_pair() {
        // 明示的に型を指定
        let pair = make_pair::<i32, &str>(100, "hello");
        assert_eq!(pair, (100, "hello"));

        // 型推論が効く場合は省略可能
        let pair = make_pair(true, 3.14);
        assert_eq!(pair, (true, 3.14));

        // 異なる型の組み合わせ
        let pair = make_pair::<String, Vec<i32>>("data".to_string(), vec![1, 2, 3]);
        assert_eq!(pair.0, "data");
        assert_eq!(pair.1, vec![1, 2, 3]);
    }

    #[test]
    fn test_multiple_trait_bounds() {
        // i32はClone + Debug + PartialOrdを実装している
        let max = print_and_clone_max(&10, &5);
        assert_eq!(max, 10);

        // Stringも同様
        let max = print_and_clone_max(&"hello".to_string(), &"world".to_string());
        assert_eq!(max, "world");

        // f64も可能
        let max = print_and_clone_max(&3.14, &2.71);
        assert_eq!(max, 3.14);
    }
}
