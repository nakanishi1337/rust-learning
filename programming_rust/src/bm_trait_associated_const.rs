// トレイトの関連定数でフィボナッチ数列を定義

use std::ops::Add;

// 数値型に必要な定数を定義するトレイト
trait NumericConstants: Sized + Copy + Add<Output = Self> {
    const ZERO: Self;
    const ONE: Self;
}

impl NumericConstants for u64 {
    const ZERO: u64 = 0;
    const ONE: u64 = 1;
}

impl NumericConstants for i32 {
    const ZERO: i32 = 0;
    const ONE: i32 = 1;
}

impl NumericConstants for f64 {
    const ZERO: f64 = 0.0;
    const ONE: f64 = 1.0;
}

// Stringは静的な定数として空文字列しか作れないので、
// 文字列フィボナッチ用の特別な関数を定義
fn string_fib(n: u32) -> String {
    match n {
        0 => "a".to_string(),
        1 => "b".to_string(),
        _ => string_fib(n - 1) + &string_fib(n - 2),
    }
}

// 任意の型に対して再帰的にフィボナッチ数列を計算
fn fib<T: NumericConstants>(n: u32) -> T {
    match n {
        0 => T::ZERO,
        1 => T::ONE,
        _ => fib::<T>(n - 1) + fib::<T>(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_u64() {
        // u64でフィボナッチ数列を計算
        assert_eq!(fib::<u64>(0), 0);
        assert_eq!(fib::<u64>(1), 1);
        assert_eq!(fib::<u64>(2), 1);
        assert_eq!(fib::<u64>(3), 2);
        assert_eq!(fib::<u64>(4), 3);
        assert_eq!(fib::<u64>(5), 5);
        assert_eq!(fib::<u64>(6), 8);
        assert_eq!(fib::<u64>(10), 55);
    }

    #[test]
    fn test_fibonacci_i32() {
        // i32でも同じ関数が使える
        assert_eq!(fib::<i32>(0), 0);
        assert_eq!(fib::<i32>(5), 5);
        assert_eq!(fib::<i32>(10), 55);
    }

    #[test]
    fn test_fibonacci_f64() {
        // f64でも計算可能（小数点の誤差に注意）
        assert_eq!(fib::<f64>(0), 0.0);
        assert_eq!(fib::<f64>(1), 1.0);
        assert_eq!(fib::<f64>(5), 5.0);
        assert_eq!(fib::<f64>(8), 21.0);
    }
}
