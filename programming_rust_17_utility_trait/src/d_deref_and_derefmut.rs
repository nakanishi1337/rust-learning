/// Deref と DerefMut の学習
///
/// Derefトレイトを実装すると、内部の値のメソッドが自動的に呼べる
/// Selector<char>として使った時、charのメソッドが呼ばれることを確認
use std::ops::{Deref, DerefMut};

struct Selector<T> {
    elements: Vec<T>,
    current: usize,
}

impl<T> Selector<T> {
    fn new(elements: Vec<T>) -> Self {
        Selector {
            elements,
            current: 0,
        }
    }
}

// Derefを実装: T をターゲットに
impl<T> Deref for Selector<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.elements[self.current]
    }
}

// DerefMutを実装
impl<T> DerefMut for Selector<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements[self.current]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deref_coercion_with_string() {
        println!("\n--- Deref Coercion: char のメソッドが自動解決 ---");

        let mut selector = Selector::new(vec!['H', '1', 'R']);

        // charのメソッドが自動的に呼べる
        assert!(selector.is_alphabetic());

        // current を変更
        selector.current = 1;
        assert!(!selector.is_alphabetic()); // '1' は英字ではない
        assert!(selector.is_numeric());

        // DerefMut で変更
        selector.current = 2;
        *selector = selector.to_ascii_uppercase();
        assert_eq!(*selector, 'R');

        println!("✅ 全テスト通過");
    }
}
