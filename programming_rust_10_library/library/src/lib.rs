//! # Math Library
//!
//! `library` は簡単な数学計算を行うためのライブラリです。

/// 2つの数値を加算します。
///
/// # Examples
///
/// ```
/// let result = library::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// 2つの数値を減算します。
///
/// # Examples
///
/// ```
/// let result = library::subtract(10, 3);
/// assert_eq!(result, 7);
/// ```
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// 2つの数値を乗算します。
///
/// # Examples
///
/// ```
/// let result = library::multiply(4, 5);
/// assert_eq!(result, 20);
/// ```
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// 2つの数値を除算します。
///
/// # Examples
///
/// ```
/// let result = library::divide(10, 2);
/// assert_eq!(result, Some(5));
/// ```
///
/// ```
/// let result = library::divide(10, 0);
/// assert_eq!(result, None);
/// ```
pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

/// 数値が偶数かどうかをチェックします。
///
/// # Examples
///
/// ```
/// assert_eq!(library::is_even(4), true);
/// assert_eq!(library::is_even(5), false);
/// ```
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
        assert_eq!(subtract(5, 5), 0);
        assert_eq!(subtract(0, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
        assert_eq!(multiply(-2, 3), -6);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Some(5));
        assert_eq!(divide(7, 3), Some(2));
        assert_eq!(divide(10, 0), None);
    }

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(4), true);
        assert_eq!(is_even(5), false);
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(-2), true);
    }
}
