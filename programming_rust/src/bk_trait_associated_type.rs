// 関連型（Associated Type）を持つトレイト

use std::fmt::Debug;

// コンテナが何を格納するかを関連型で定義
trait Container {
    type Item; // 関連型：実装側が具体的な型を指定
    fn get(&self) -> &Self::Item;
}

struct IntBox(i32);
struct StringBox(String);

impl Container for IntBox {
    type Item = i32; // IntBoxは i32 を格納
    fn get(&self) -> &Self::Item {
        &self.0
    }
}

impl Container for StringBox {
    type Item = String; // StringBox は String を格納
    fn get(&self) -> &Self::Item {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_type() {
        let int_box = IntBox(42);
        let string_box = StringBox("hello".to_string());

        // 各型が定義した関連型が使われる
        assert_eq!(int_box.get(), &42);
        assert_eq!(string_box.get(), &"hello".to_string());
    }

    #[test]
    fn test_generic_function_with_associated_type() {
        // 関連型を使ったジェネリック関数
        fn print_item<C: Container>(container: &C)
        where
            C::Item: Debug,
        {
            println!("Container has item: {:?}", container.get());
        }

        let int_box = IntBox(100);
        print_item(&int_box);

        let string_box = StringBox("world".to_string());
        print_item(&string_box);
    }

    #[test]
    fn test_why_associated_type_is_better_than_generic() {
        // ❌ ジェネリックパラメータ版
        // trait Container<Item> { fn get(&self) -> &Item; }
        // impl Container<i32> for MyBox { ... }
        // impl Container<String> for MyBox { ... }  ← 同じMyBoxに複数実装可能
        // → 実際にはこういうケースは少ない！

        // ✅ 関連型版（ほとんどのケースで推奨）
        // trait Container { type Item; fn get(&self) -> &Self::Item; }
        // impl Container for IntBox { type Item = i32; ... }
        // → IntBoxは常にi32を格納する（一対一の関係）
        // → 「1つの型に複数のバージョンを実装したい」というケースは稀
        // → 関連型の方がシンプルで意図が明確

        let int_box = IntBox(42);
        let item = int_box.get(); // Itemは確実にi32
        assert_eq!(*item, 42);

        // 結論: 「この型はこのデータを格納する」という一対一の関係なら関連型を使う
        // ジェネリックは「同じ型で複数のバージョンが必要」な稀なケースのみ
    }
}
