// ジェネリック関数で明示的に型を指定する必要がある場合

use std::str::FromStr;

// 文字列をパースする汎用関数
fn parse<T: FromStr>(s: &str) -> Option<T> {
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit_type_needed() {
        // ❌ コンパイルエラー: 型が推論できない
        // let num = parse("42");

        // ✅ 方法1: 変数の型を指定
        let num: Option<i32> = parse("42");
        assert_eq!(num, Some(42));

        // ✅ 方法2: ターボフィッシュ構文 ::<>
        let num = parse::<i32>("42");
        assert_eq!(num, Some(42));

        // ✅ 方法3: 使用時に型が明らか
        let num = parse("42").unwrap_or(0_i32);
        assert_eq!(num, 42);
    }

    #[test]
    fn test_multiple_possible_types() {
        // 同じ文字列を異なる型にパース
        assert_eq!(parse::<i32>("100"), Some(100));
        assert_eq!(parse::<f64>("100"), Some(100.0));
        assert_eq!(parse::<String>("100"), Some("100".to_string()));
    }

    #[test]
    fn test_collect_needs_type() {
        let numbers = vec![1, 2, 3];

        // ❌ コンパイルエラー: collectの結果の型が推論できない
        // let doubled = numbers.iter().map(|x| x * 2).collect();

        // ✅ 明示的に型を指定
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);

        // ✅ ターボフィッシュでも可能
        let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<i32>>();
        assert_eq!(doubled, vec![2, 4, 6]);
    }

    #[test]
    fn test_empty_collection() {
        // ❌ コンパイルエラー: 空のコレクションは型を推論できない
        // let v = Vec::new();

        // ✅ 型を明示
        let v: Vec<i32> = Vec::new();
        assert_eq!(v.len(), 0);

        // ✅ ターボフィッシュでも可能
        let v = Vec::<i32>::new();
        assert_eq!(v.len(), 0);
    }
}
