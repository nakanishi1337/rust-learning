
// 単に+演算子を使うとオーバーフロー時にパニックが発生する
// let _ = 200_u8 + 100;

// checked系のメソッドを使うと、Noneを利用したオプショナルな結果が返される
#[test]
fn test_checked_operation() {
    // 数学的に正しい答えが得られる場合
    assert!(10_u8.checked_add(20) == Some(30));
    // オーバーフローが発生する場合
    assert!(200_u8.checked_add(100) == None);
}

// wrapping系のメソッドを使うと、オーバーフロー時に値がラップアラウンドされる
#[test]
fn test_wrapped_operation() {
    assert_eq!(100_u16.wrapping_mul(200), 20000);
    assert_eq!(500_u16.wrapping_mul(500), 53392);
    assert_eq!(500_i16.wrapping_mul(500), -12144);
}

// saturating系のメソッドを使うと、オーバーフロー時に最大値または最小値に固定される
#[test]
fn test_saturating_operation() {
    assert_eq!(32760_i16.saturating_add(10), 32767);
    assert_eq!((-32760_i16).saturating_sub(10), -32768);
}

// overflowing系のメソッドを使うと、タプルで結果とオーバーフローの有無が返される
#[test]
fn test_overflowing_operation() {
    assert_eq!(255_u8.overflowing_sub(2), (253, false));
    assert_eq!(255_u8.overflowing_add(2), (1, true));
}