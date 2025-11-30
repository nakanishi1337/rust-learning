fn main() {
    example();
    example_2();
    example_3();
    example_4();
    // example_5();
}

// This function demonstrates borrowing a reference to a String
fn example() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// This function demonstrates borrowing a reference to a String
// and shows that the reference is immutable.
// Uncommenting the line that tries to modify the string will cause a compile-time error.
fn example_2() {
    let s = String::from("hello");

    change(&s);
}
fn change(some_string: &String) {
    // some_string.push_str(", world");  // This line would cause a compile-time error because `some_string` is an immutable reference
}

// This function demonstrates mutable borrowing.
// It allows the string to be modified through a mutable reference.
// Uncommenting the line that tries to modify the string will work correctly.
fn example_3() {
    let mut s = String::from("hello");

    change_mut(&mut s);
}

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}


fn example_4() {
let mut s = String::from("hello");

    let _r1 = &s; // 問題なし
    let _r2 = &s; // 問題なし
    let _r3 = &mut s; // 自動で開放しr3のみ
    // println!("{}, {}, and {}", r1, r2, r3); // ここでコンパイルエラーが発生する
    // これは、同時に不変参照と可変参照を持つことができないためです。
}

fn example_5() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s // これはコンパイルエラーになります。sはこの関数のスコープを抜けると無効になるため。
}