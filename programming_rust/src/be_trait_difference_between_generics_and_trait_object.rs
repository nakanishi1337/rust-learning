// ジェネリック vs トレイトオブジェクト

trait Speak {
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Speak for Dog {
    fn speak(&self) -> String {
        "Woof!".to_string()
    }
}

impl Speak for Cat {
    fn speak(&self) -> String {
        "Meow!".to_string()
    }
}

// ジェネリック: コンパイル時に型が決定（静的ディスパッチ）
// Dog用とCat用に別々のコードが生成される
fn generic_speak<T: Speak>(animal: &T) -> String {
    animal.speak()
}

// トレイトオブジェクト: 実行時に型が決定（動的ディスパッチ）
// 1つのコードで全ての型を扱える
fn trait_object_speak(animal: &dyn Speak) -> String {
    animal.speak()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generic() {
        let dog = Dog;
        let cat = Cat;

        // ジェネリック: 各型ごとにコンパイル時に特殊化
        assert_eq!(generic_speak(&dog), "Woof!");
        assert_eq!(generic_speak(&cat), "Meow!");
    }

    #[test]
    fn test_trait_object() {
        let dog = Dog;
        let cat = Cat;

        // トレイトオブジェクト: 実行時にvtable経由で呼び出し
        assert_eq!(trait_object_speak(&dog), "Woof!");
        assert_eq!(trait_object_speak(&cat), "Meow!");
    }

    #[test]
    fn test_heterogeneous_collection() {
        // ジェネリック: 異なる型を同じコレクションに入れられない
        // let animals: Vec<&T> = vec![&Dog, &Cat]; // コンパイルエラー！

        // トレイトオブジェクト: 異なる型を同じコレクションに入れられる
        let animals: Vec<&dyn Speak> = vec![&Dog, &Cat];
        let results: Vec<String> = animals.iter().map(|a| a.speak()).collect();

        assert_eq!(results, vec!["Woof!", "Meow!"]);
    }
}
