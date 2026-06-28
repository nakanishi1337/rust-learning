// 式は値を返す
// ステートメントは値を返さない
// c言語系言語では、ifやmatchはステートメントとして扱われ値を返さないが、Rustでは式として扱われるため値を返す

#[cfg(test)]
mod tests {
    // ifは式: 値を返すので、そのまま変数に代入できる
    #[test]
    fn if_is_expression() {
        let flag = true;
        let x = if flag { 10 } else { 20 };
        assert_eq!(x, 10);
    }

    // matchも式: アームの結果が同じ型なら値として使える
    #[test]
    fn match_is_expression() {
        let y = match Some(7) {
            Some(v) => v,
            None => 0,
        };
        assert_eq!(y, 7);
    }
}
