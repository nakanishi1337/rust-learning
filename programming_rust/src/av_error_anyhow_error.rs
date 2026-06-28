use anyhow::{Context, Result, bail};

// anyhowの利点：
// - 複数の異なるエラー型を Result<T> で統一的に扱える
// - .context() でエラーに分かりやすいメッセージを追加
// - bail!() でカスタムエラーを簡単に作成
// - 呼び出し側で エラー型を気にせず処理できる

#[cfg(test)]
mod tests {
    use super::*;

    // ParseIntErrorを返す関数
    fn parse_number(s: &str) -> Result<i32> {
        // ParseIntError が自動的に anyhow::Error に変換される
        s.parse::<i32>().context("Failed to parse integer")
    }

    // カスタムエラーを返す関数
    fn validate_range(n: i32) -> Result<()> {
        if n < 0 || n > 100 {
            bail!("Number out of range: {}", n);
        }
        Ok(())
    }

    // 異なるエラー型を含む関数
    // parse_number から ParseIntError が来ても
    // validate_range から bail! が来ても、
    // どちらも処理できる
    fn process_input(s: &str) -> Result<String> {
        let num = parse_number(s)?;
        validate_range(num)?;
        Ok(format!("Valid number: {}", num))
    }

    #[test]
    fn anyhow_example() {
        // 成功例
        match process_input("42") {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("Error: {:?}", e),
        }

        // ParseIntError を処理
        println!("\n--- Parse error ---");
        match process_input("abc") {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("Error: {:?}", e),
        }

        // カスタムエラー（bail!）を処理
        println!("\n--- Range error ---");
        match process_input("150") {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
