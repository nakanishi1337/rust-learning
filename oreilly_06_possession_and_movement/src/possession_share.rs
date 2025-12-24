use std::{clone, rc::Rc};

#[test]
fn possession_share() {
    // 文字列に対するcloneは単なるディープコピー
    let u = "world".to_string();
    let u1 = u.clone();

    // Rc<T>を使うと、所有権を共有できる
    let s = Rc::new("hello".to_string());
    let s1 = s.clone();
    let s2 = s.clone();
}