/// Rustでは*で明示的に解決する必要があることを示す例

#[test]
fn explicit_dereference_required() {
    let x = 5;
    let r = &x;  // xへの参照を作成
    
    // 参照をそのまま使うことはできない
    // assert_eq!(5, r);  // コンパイルエラー: `i32` と `&i32` は比較できない
    
    // 明示的に*で解決(デリファレンス)する必要がある
    assert_eq!(5, *r);  // OK! *rで参照を解決して値を取得
}

#[test]
fn dereference_with_operations() {
    let x = 10;
    let r = &x;
    
    // 参照に対して直接演算はできない
    // let y = r + 1;  // コンパイルエラー
    
    // *で解決してから演算
    let y = *r + 1;  // OK! 11
    assert_eq!(y, 11);
}

#[test]
fn mutable_reference_requires_dereference() {
    let mut x = 5;
    let r = &mut x;
    
    // 参照をそのまま変更することはできない    
    // 明示的に*で解決して変更
    *r = 10;
    assert_eq!(x, 10);
}

#[test]
fn no_auto_dereference_in_comparison() {
    let values = vec![1, 2, 3];
    let first_ref = &values[0];
    
    // 参照と値の比較は自動で解決されない場合がある
    // assert_eq!(1, first_ref);  // コンパイルエラー
    
    // 明示的に*で解決
    assert_eq!(1, *first_ref);  // OK!
}

/// ドット演算子(.)を使えば暗黙的にデリファレンスされることを示す例

#[test]
fn implicit_dereference_with_dot_operator() {
    let x = String::from("Hello");
    let r = &x;  // String への参照
    
    // .を使うと暗黙的にデリファレンスされる
    // r.len() は (*r).len() と同じ意味
    assert_eq!(r.len(), 5);  // OK! 自動的に解決される
    assert_eq!((*r).len(), 5);  // これと同じ
}

#[test]
fn implicit_dereference_with_struct() {
    struct Person {
        name: String,
        age: u32,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    let person_ref = &person;
    
    // .を使ってフィールドにアクセス
    // person_ref.name は (*person_ref).name と同じ
    assert_eq!(person_ref.name, "Alice");
    assert_eq!(person_ref.age, 30);
}
