fn main() {
    example();
    example2();
    // example3();
    example4();
}

fn example() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5
                               // wordの中身は、値5になる

    s.clear(); // this empties the String, making it equal to ""
               // Stringを空にする。つまり、""と等しくする

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
    // wordはまだ値5を保持しているが、もうこの値を正しい意味で使用できる文字列は存在しない。
    // wordは今や完全に無効なのだ！
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn example2(){
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}

fn example3() {
    let mut s = String::from("hello world");

    let word = first_word_slice(&s); // word will get the value 5
                               // wordの中身は、値5になる

    // s.clear(); //error
    println!("the first word is: {}", word);
}

fn first_word_slice(s: &String) -> &str {    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn example4() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    // first_wordは`String`のスライスに対して機能する
    let word = first_word_update(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    // first_wordは文字列リテラルのスライスに対して機能する
    let word = first_word_update(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // 文字列リテラルは「それ自体すでに文字列スライスなので」、
    // スライス記法なしでも機能するのだ！
    let word = first_word_update(my_string_literal);
}

fn first_word_update(s: &str) -> &str {    
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}