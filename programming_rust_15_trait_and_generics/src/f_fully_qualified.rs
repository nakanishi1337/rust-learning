// 完全修飾パスと呼び出し方の違い

use std::string::ToString;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_ways_to_call_to_string() {
        // 1️⃣ メソッド呼び出し
        // "hello"が&str型で、その型のメソッドを呼び出す
        // シンタックスシュガーであり、3️⃣や4️⃣に変換される
        assert_eq!("hello".to_string(), "hello");

        // 2️⃣ 型の完全修飾パス（型 namespace でのメソッド呼び出し）
        // str型に定義された型関連関数を呼び出す
        assert_eq!(str::to_string("hello"), "hello");

        // 3️⃣ トレイト名を明示（トレイト namespace でのメソッド呼び出し）
        // ToString トレイトに定義された型関連関数を呼び出す
        assert_eq!(ToString::to_string("hello"), "hello");

        // 4️⃣ 型とトレイトを両方明示（完全修飾パス <T as Trait>）
        // 最も詳細で、型推論の余地がない
        // コンパイラが型を勝手に推論しない（明示的で曖昧性なし）
        assert_eq!(<str as ToString>::to_string("hello"), "hello");
    }

    #[test]
    fn test_when_fully_qualified_is_needed() {
        // 複数の型で同じメソッド名がある場合、完全修飾パスで型を指定することでデバッグになるよ
        assert_eq!(<i32 as ToString>::to_string(&100), "100");
        assert_eq!(<bool as ToString>::to_string(&true), "true");
        assert_eq!(<f64 as ToString>::to_string(&3.14), "3.14");
    }
}
