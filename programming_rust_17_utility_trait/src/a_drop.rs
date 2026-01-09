/// Dropトレイトの学習
///
/// Dropトレイトは、値がスコープから外れるときに実行されるカスタムコードを定義できる

struct Droppable {
    name: String,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("🗑️  '{}' をドロップしています！", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_drop() {
        println!("\n--- test_basic_drop 開始 ---");
        {
            let x = Droppable {
                name: String::from("変数x"),
            };
            println!("変数xを作成しました");
            // スコープの終わりでxがドロップされる
        }
        println!("スコープ終了");
        println!("--- test_basic_drop 終了 ---\n");
    }

    #[test]
    fn test_multiple_drops() {
        println!("\n--- test_multiple_drops 開始 ---");
        {
            let first = Droppable {
                name: String::from("最初"),
            };
            let second = Droppable {
                name: String::from("2番目"),
            };
            let third = Droppable {
                name: String::from("3番目"),
            };

            println!("3つの変数を作成しました");
            // スコープの終わりで逆順(LIFO)にドロップされる
            // 3番目 → 2番目 → 最初
        }
        println!("--- test_multiple_drops 終了 ---\n");
    }

    #[test]
    fn test_explicit_drop() {
        println!("\n--- test_explicit_drop 開始 ---");
        {
            let x = Droppable {
                name: String::from("明示的にドロップ"),
            };
            println!("変数xを作成しました");

            // std::mem::dropを使って明示的にドロップ
            drop(x);
            println!("dropを呼び出した後");

            // ここでxはもう使えない（所有権が移動済み）
        }
        println!("--- test_explicit_drop 終了 ---\n");
    }
}
