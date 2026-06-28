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
    let value = no_dangle();
    println!("{}", value);
}

fn no_dangle() -> String {
    String::from("hello")
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_runs() {
        main();
        example_5();
    }

    #[test]
    fn calculate_string_length_by_reference() {
        let s = String::from("hello");

        assert_eq!(calculate_length(&s), 5);
        assert_eq!(s, "hello");
    }

    #[test]
    fn mutable_reference_changes_string() {
        let mut s = String::from("hello");

        change_mut(&mut s);

        assert_eq!(s, "hello, world");
    }

    #[test]
    fn return_owned_value_instead_of_dangling_reference() {
        assert_eq!(no_dangle(), "hello");
    }
}
