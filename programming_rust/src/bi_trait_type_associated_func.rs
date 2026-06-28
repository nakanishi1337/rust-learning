// 型関連関数とトレイトオブジェクト

trait Repository {
    fn create(&self, data: &str) -> String; // メソッド
}

struct UserRepo;
struct ProductRepo;

impl Repository for UserRepo {
    fn create(&self, data: &str) -> String {
        format!("User created: {}", data)
    }
}

impl Repository for ProductRepo {
    fn create(&self, data: &str) -> String {
        format!("Product created: {}", data)
    }
}

impl UserRepo {
    // 型関連関数：型に紐付けられたコンストラクタ
    fn new() -> Self {
        UserRepo
    }
}

impl ProductRepo {
    // 型関連関数：型に紐付けられたコンストラクタ
    fn new() -> Self {
        ProductRepo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_function_with_concrete_types() {
        // ✅ 具体的な型では型関連関数が使える
        let user_repo = UserRepo::new();
        assert_eq!(user_repo.create("Alice"), "User created: Alice");

        let product_repo = ProductRepo::new();
        assert_eq!(product_repo.create("Widget"), "Product created: Widget");
    }

    #[test]
    fn test_trait_object_cannot_use_associated_function() {
        // ✅ トレイトオブジェクトではメソッドは使える
        let user_repo = UserRepo::new();
        let product_repo = ProductRepo::new();

        let repos: Vec<&dyn Repository> = vec![&user_repo, &product_repo];
        for repo in repos {
            println!("{}", repo.create("test"));
        }

        // ❌ こういったことはできない
        // let obj: &dyn Repository = /* ... */;
        // obj.new();  // エラー！型関連関数は呼べない

        // 理由: トレイトオブジェクト（&dyn Repository）はコンパイル時に具体的な型が不明なため、
        // どの型の new() を呼ぶべきかわからない
        // Self を返すため、戻り値の型も不明になる
    }
}
