
// 例外: コピー可能な型はムーブされない
#[test]
fn exception_of_movement() {
    let string1 = "hello".to_string();
    let string2 = string1;
    // println!("{}", string1); // コンパイルエラー

    let num1 = 5;
    let num2 = num1;
    println!("{}", num1); // 問題なく動作
}

#[test]
fn struct_copyable(){
    // structはデフォルトでCopyトレイトを実装していないが、
    // Copyトレイトを実装することでムーブされなくなる
    #[derive(Copy, Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // PointはCopyトレイトを実装しているのでムーブされない
    println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y); // 問題なく動作
}

#[test]
fn struct_not_copyable(){
    // structはデフォルトでCopyトレイトを実装していない
    // ヒープ領域を使用するString型をフィールドに持つ場合、Copyトレイトを実装できない
    struct Point {
        x: String,
        y: String,
    }
    let p1 = Point { x: "10".to_string(), y: "20".to_string() };
    let p2 = p1;
    // println!("p1: ({}, {}), p2: ({}, {})", p1.x, p1.y, p2.x, p2.y); // コンパイルエラー
}

// Rustとしては、後々に非コピー型になる可能性があると、
// コピー型から非コピー型への変更は難しいため、
// 最初から非コピー型として設計することを推奨している。