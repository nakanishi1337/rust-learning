/// Clone と Copy の違いの学習
///
/// Copy: 暗黙的なコピー（代入で自動的にコピーされる）- スタック上の単純な型のみ
/// Clone: 明示的なコピー（.clone()を呼ぶ必要がある）- ヒープメモリを持つ型でも可能

// Copy型: 代入で自動的にコピーされる
#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

// Clone型: .clone()で明示的にコピー（ヒープメモリを持つ）
#[derive(Debug, Clone)]
struct Person {
    name: String, // String はヒープメモリを持つので Copy 不可
    age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_vs_clone() {
        println!("\n--- Copy vs Clone ---");

        // Copy: 代入で自動的にコピー
        let p1 = Point { x: 10, y: 20 };
        let p2 = p1; // 暗黙的にコピー
        println!("📋 Copy: p1 = {:?}, p2 = {:?}", p1, p2);
        assert_eq!(p1.x, 10); // p1 はまだ使える！

        // Clone: .clone()で明示的にコピー
        let person1 = Person {
            name: String::from("太郎"),
            age: 25,
        };
        let person2 = person1.clone(); // 明示的にクローン
        println!("🧬 Clone: person1 = {:?}", person1); // person1 はまだ使える
        println!("🧬 Clone: person2 = {:?}", person2);

        // Clone なしで代入すると所有権が移動
        let person3 = Person {
            name: String::from("花子"),
            age: 30,
        };
        let person4 = person3; // 所有権が移動
                               // println!("{:?}", person3); // エラー！person3 はもう使えない
        println!("📦 Move: person4 = {:?}", person4);

        assert_eq!(person2.age, 25);
        assert_eq!(person4.age, 30);
    }
}
