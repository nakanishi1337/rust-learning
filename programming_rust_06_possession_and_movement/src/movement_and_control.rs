fn f (vector: Vec<i32>) -> Vec<i32> {
    vector.iter().map(|x| x + 1).collect()
}

#[test]
fn movement_and_if() {
    let x = vec![1, 2, 3];
    if true {
        f(x);
    } else {
        f(x);
    }
    // f(x); // いずれかの経路でムーブされてると、ここでの使用はコンパイルエラー
}

# [test]
fn movement_and_loop() {
    let x = vec![1, 2, 3];
    for _ in 0..2 {
        // f(x); // コンパイルエラー: ループの最初の反復でムーブされる
    }
}

#[test]
fn movement_and_loop2() {
    let mut x = vec![1, 2, 3];
    for _ in 0..2 {
        let y = f(x); 
        x = y; // 次のイテレーション前に新しい値があればOk
    }
}