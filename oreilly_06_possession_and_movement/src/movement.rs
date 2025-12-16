// ヒープを参照するデータのムーブ
// #[test]
// fn movement() {
//     let s = ["udon".to_string(), "ramen".to_string(), "soba".to_string()];
//     let t = s; // sはヒープ領域のデータを指しているため、ムーブされる
//     let u = s; // コンパイルエラー: sはすでにムーブされている
// }

// ヒープ自体をディープコピー
#[test]
fn deep_copy() {
    let s = ["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s.clone(); // cloneメソッドを使ってディープコピーを作成
    let u = s.clone(); // これは問題ない: sはまだ有効
}

// 代入
#[test]
fn assign() {
    let mut s = "Govinda".to_string();
    s = "Siddhartha".to_string(); // ヒープ領域のデータは新しいデータで上書きされる
}

// 代入ではなく初期化
#[test]
fn init_assign() {
    let s = "Govinda".to_string();
    let t = s; // sはムーブされる
    let s = "Siddhartha".to_string(); // これは新しい変数の初期化
}

