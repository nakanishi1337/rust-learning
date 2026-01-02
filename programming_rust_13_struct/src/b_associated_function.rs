// 関連関数（Associated Functions）
// - 型関連関数：型に紐づく関数、Struct::function() で呼び出す
// - メソッド：インスタンスに紐づく関数、instance.method() で呼び出す

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // 型関連関数：self を受け取らない
    // Rectangle::new() で呼び出す
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // メソッド：&self を受け取る（イミュータブル）
    // rect.area() で呼び出す
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // メソッド：&mut self を受け取る（ミュータブル）
    // rect.resize() で呼び出す
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    // メソッド：self を受け取る（所有権を移動）
    // rect.into_description() で呼び出すと rect は消費される
    fn into_description(self) -> String {
        format!("幅={}, 高さ={}", self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_associated_function() {
        // 型関連関数は Struct::function() で呼び出す
        let rect = Rectangle::new(10, 20);

        println!("型関連関数で作成: 幅={}, 高さ={}", rect.width, rect.height);
        assert_eq!(rect.width, 10);
        assert_eq!(rect.height, 20);
    }

    #[test]
    fn immutable_method() {
        let rect = Rectangle::new(5, 8);

        // メソッド（&self）はインスタンスから呼び出す
        let area = rect.area();
        println!("面積（イミュータブルメソッド）: {}", area);
        assert_eq!(area, 40);
    }

    #[test]
    fn mutable_method() {
        let mut rect = Rectangle::new(5, 8);

        println!("変更前: 幅={}, 高さ={}", rect.width, rect.height);

        // メソッド（&mut self）はミュータブルなインスタンスから呼び出す
        rect.resize(15, 10);
        println!("変更後: 幅={}, 高さ={}", rect.width, rect.height);

        assert_eq!(rect.width, 15);
        assert_eq!(rect.height, 10);
    }

    #[test]
    fn consuming_method() {
        let rect = Rectangle::new(5, 8);

        // メソッド（self）はインスタンスの所有権を移動（消費）する
        // このメソッド呼び出し後は rect は使えない
        let description = rect.into_description();
        println!("説明（所有権を移動するメソッド）: {}", description);
        assert_eq!(description, "幅=5, 高さ=8");

        // rect.area() はエラー！rectはもう存在しない
        // println!("{}", rect.width);  // コンパイルエラー
    }
}
