use std::vec;

#[test]
fn movement_and_vector_index_reference() {
    let mut v = vec![1.to_string(), 2.to_string(), 3.to_string()];
    // let r = v[2]; // ムーブ扱いだが、ベクタから値を取り出すことはできないため、コンパイルエラーになる
}

#[test]
fn vector_pop() {
    let mut v = vec![1.to_string(), 2.to_string(), 3.to_string()];
    let r = v.pop(); // popはムーブ扱いで値を取り出せる
    assert_eq!(r, Some("3".to_string()));
    assert_eq!(v, vec!["1".to_string(), "2".to_string()]);
}

#[test]
fn vector_loop() {
    let v = vec![1.to_string(), 2.to_string(), 3.to_string()];
    for s in v { // ベクタの所有権がムーブする
        println!("{}", s);
    }
    // vは未初期化状態になる
}

#[test]
fn vector_loop_ref() {
    let v = vec![1.to_string(), 2.to_string(), 3.to_string()];
    for s in &v { // ベクタの所有権がムーブする
        println!("{}", s);
    }
    assert_eq!(v, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
}

#[test]
fn what_is_take() {
    let mut v = vec![1.to_string(), 2.to_string(), 3.to_string()];
    let r = std::mem::take(&mut v); // ベクタの所有権をムーブし、元のベクタは空になる
    assert_eq!(r, vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    assert_eq!(v, Vec::<String>::new());

    // takeは元のベクタを空にしたり、OptionをNoneにしたりして、ムーブを可能にする
}