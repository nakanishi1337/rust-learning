// 別プロジェクトから library クレートを使う例
use library::{add, subtract, multiply, divide, is_even};

fn main() {
    println!("=== 外部ライブラリとして library クレートを使用 ===\n");

    // 計算例
    let a = 100;
    let b = 25;

    println!("a = {}, b = {}\n", a, b);
    println!("加算: {} + {} = {}", a, b, add(a, b));
    println!("減算: {} - {} = {}", a, b, subtract(a, b));
    println!("乗算: {} × {} = {}", a, b, multiply(a, b));
    
    match divide(a, b) {
        Some(result) => println!("除算: {} ÷ {} = {}", a, b, result),
        None => println!("除算エラー"),
    }

    println!("\n偶数判定:");
    for n in [10, 15, 20, 33] {
        println!("  {} は偶数? {}", n, is_even(n));
    }
}
