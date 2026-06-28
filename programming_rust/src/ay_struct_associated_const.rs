// 型関連定数（Associated Constants）
// 型に紐づく定数、Type::CONST で呼び出す

struct Circle {
    radius: f64,
}

impl Circle {
    // 型関連定数：const キーワードで定義
    // Circle::PI で呼び出す
    const PI: f64 = 3.14159;
    const ZERO_RADIUS: f64 = 0.0;

    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // 型関連定数を使うメソッド
    fn circumference(&self) -> f64 {
        2.0 * Self::PI * self.radius // Self::PI で型関連定数を参照
    }

    fn area(&self) -> f64 {
        Self::PI * self.radius * self.radius
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_associated_const() {
        // 型関連定数は Type::CONST で呼び出す
        println!("PI = {}", Circle::PI);
        println!("ZERO_RADIUS = {}", Circle::ZERO_RADIUS);

        assert_eq!(Circle::PI, 3.14159);
        assert_eq!(Circle::ZERO_RADIUS, 0.0);
    }

    #[test]
    fn use_associated_const_in_method() {
        let circle = Circle::new(5.0);

        // メソッド内で Self::PI を使っている
        let circumference = circle.circumference();
        let area = circle.area();

        println!("半径 5 の円");
        println!("円周: {:.2}", circumference);
        println!("面積: {:.2}", area);

        assert!((circumference - 31.4159).abs() < 0.01);
        assert!((area - 78.5398).abs() < 0.01);
    }
}
