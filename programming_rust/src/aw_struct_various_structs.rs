// 1. 名前付き構造体（Named Struct）
struct Person {
    name: String,
    age: u32,
}

// 2. タプル構造体（Tuple Struct）
struct Point(i32, i32);

// 3. ユニット型構造体（Unit-like Struct）
struct Marker;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn named_struct() {
        let person = Person {
            name: String::from("太郎"),
            age: 25,
        };

        println!("名前: {}, 年齢: {}", person.name, person.age);
        assert_eq!(person.name, "太郎");
        assert_eq!(person.age, 25);
    }

    #[test]
    fn tuple_struct() {
        let point = Point(10, 20);

        println!("x: {}, y: {}", point.0, point.1);
        assert_eq!(point.0, 10);
        assert_eq!(point.1, 20);
    }

    #[test]
    // 用途については今は特に説明しない
    fn unit_struct() {
        let marker = Marker;

        println!("マーカーを作成しました");
        // ユニット型構造体はデータを持たず、型としての役割のみ
        let _another = Marker;
    }
}
