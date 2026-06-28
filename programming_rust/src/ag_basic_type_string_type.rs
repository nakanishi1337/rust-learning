
#[test]
fn string_literal() {
    let s: &str = "\"Hello, world!\"";
    assert_eq!(s, "\"Hello, world!\"");
}

#[test]
fn raw_string_literal() {
    let s: &str = r#"This is a "raw" string with \n no escapes."#;
    assert_eq!(s, r#"This is a "raw" string with \n no escapes."#);

    let s: &str = r###"This is a "raw" string with \n no escapes.  You can use "#. "###;
    assert_eq!(s, r###"This is a "raw" string with \n no escapes.  You can use "#. "###);
}

#[test]
fn byte_string() {
    // ascii only
    let b: &[u8] = b"Hello, world!";
    assert_eq!(b, &[b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!']);
    assert_eq!(b, &[72, 101, 108, 108, 111, 44, 32, 119, 111, 114, 108, 100, 33]);
}

#[test]
fn test_dynamic_string() {
    let s1: &str = "Hello";
    let s2: String = s1.to_string(); //　コピーして動的なString型に変換
    assert_eq!(s2, "Hello"); // String型は&strと比較可能
}

#[test]
fn test_format() {
    let name: &str = "Alice";
    let age: u8 = 30;
    let s: String = format!("Name: {}, Age: {}", name, age); // println!と同じフォーマット指定子が使えるString作成方法
    assert_eq!(s, "Name: Alice, Age: 30");
}

#[test]
fn string_concatenation() {
    let s1: String = String::from("Hello, ");
    let s2: &str = "world!";
    let s3: String = s1 + s2; // s1はムーブされる
    assert_eq!(s3, "Hello, world!");
}