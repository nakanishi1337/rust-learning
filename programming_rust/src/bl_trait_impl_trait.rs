// impl Trait 構文（特に戻り値で威力を発揮）

use std::fmt::Display;

// 引数で impl Trait を使う例
fn print_twice(value: impl Display) {
    println!("{}", value);
    println!("{}", value);
}

// 🔥 戻り値で impl Trait の真価を発揮する例
// 複雑なイテレータ型を隠蔽できる
fn get_positive_numbers(numbers: Vec<i32>) -> impl Iterator<Item = i32> {
    // 具体的な型: std::iter::Filter<std::vec::IntoIter<i32>, [closure]>
    // → こんな長い型を書かなくて済む！
    numbers.into_iter().filter(|x| *x > 0)
}

// 内部実装を変更しても、外部APIは変わらない
fn get_positive_numbers_v2(numbers: Vec<i32>) -> impl Iterator<Item = i32> {
    // 実装を map を追加するように変更
    // → 戻り値の型は変わらない（impl Iterator のまま）
    numbers.into_iter().filter(|x| *x > 0).map(|x| x * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_impl_trait_as_argument() {
        print_twice(42);
        print_twice("hello");
    }

    #[test]
    fn test_impl_trait_hides_complex_return_type() {
        // ✅ 戻り値の複雑な型を知る必要がない
        // std::iter::Filter<std::vec::IntoIter<i32>, [closure]> を書かなくて済む
        let numbers = vec![-2, -1, 0, 1, 2, 3];
        let positive: Vec<i32> = get_positive_numbers(numbers).collect();
        assert_eq!(positive, vec![1, 2, 3]);
    }

    #[test]
    fn test_impl_trait_allows_implementation_change() {
        // 🔥 内部実装が変わっても（filter → filter + map）、
        // 呼び出し側のコードは変わらない（impl Iterator のまま）
        let numbers = vec![-2, -1, 0, 1, 2, 3];
        let positive: Vec<i32> = get_positive_numbers_v2(numbers).collect();
        assert_eq!(positive, vec![2, 4, 6]); // *2された結果

        // もし戻り値を具体的な型で書いていたら、
        // 実装変更のたびに型シグネチャが変わってしまう！
    }
}
