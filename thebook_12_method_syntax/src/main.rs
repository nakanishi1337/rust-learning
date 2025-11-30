fn main() {
    example();
    example2();
    example3();
    example4();
}

fn example(){
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // Method to calculate the area of the rectangle
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
        
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

fn example2() {
    // Point構造体の定義
    struct Point {
        x: i32,
        y: i32,
    }

    // Point構造体へのメソッド実装
    impl Point {
        // &self (不変参照) を受け取るメソッド
        fn print_coords(&self) {
            println!("座標: ({}, {})", self.x, self.y);
        }

        // &mut self (可変参照) を受け取るメソッド
        fn move_by(&mut self, dx: i32, dy: i32) {
            self.x += dx;
            self.y += dy;
            println!("新しい座標: ({}, {})", self.x, self.y);
        }

        // self (所有権) を受け取るメソッド
        fn into_string(self) -> String {
            format!("Point({}, {})", self.x, self.y)
        }
    }

    // --- メインロジック ---

    let p1 = Point { x: 10, y: 20 };
    let mut p2 = Point { x: 30, y: 40 };
    let p3 = Point { x: 50, y: 60 };

    println!("--- 自動参照の例 ---");

    // p1はPoint型。print_coordsは&selfを受け取るが、コンパイラが自動で&p1に変換します。
    // C/C++であれば、ポインタでない限り通常は `p1.print_coords()` ですが、
    // ポインタであれば `(&p1)->print_coords()` のような明示的な操作が必要になることもあります。
    p1.print_coords();

    // p2はPoint型。move_byは&mut selfを受け取りますが、コンパイラが自動で&mut p2に変換します。
    p2.move_by(5, -10);

    println!("\n--- 自動逆参照の例 (ポインタのような動作) ---");

    let p_ref: &Point = &p3; // p3への不変参照（C/C++のポインタに近い概念）
    // p_refは`&Point`型。print_coordsは`&self`を受け取りますが、
    // Rustコンパイラは`p_ref`を自動で逆参照し（`*p_ref`のように）、さらに参照を取って（`&*p_ref`のように）メソッドに渡します。
    // C/C++であれば、このような場合は `p_ref->print_coords()` と書くことが多いでしょう。
    p_ref.print_coords();

    println!("\n--- 所有権を受け取るメソッド ---");
    let p4 = Point { x: 70, y: 80 };
    // into_stringはself（所有権）を受け取ります。
    // メソッド呼び出し後、p4の所有権はムーブされるため、この行以降でp4を使用することはできません。
    let p4_str = p4.into_string();
    println!("p4の文字列表現: {}", p4_str);
    // 次の行は、p4がムーブされた後に使用しようとしているためコンパイルエラーになります。
    // println!("{:?}", p4); // <- エラーになることを確認するためのコメント
}

fn example3() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

     let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    // rect1にrect2ははまり込む？
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

}

fn example4(){
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle { width: size, height: size }
        }
    }
    
    let sq = Rectangle::square(3);
    println!("Square: {:?}", sq);
}