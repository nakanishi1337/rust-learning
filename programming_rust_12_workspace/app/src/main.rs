use utils::{add, greet};

fn main() {
    println!("{}", greet("Rustプログラマー"));

    let result = add(5, 3);
    println!("5 + 3 = {}", result);
}
