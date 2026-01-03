// matchにおける ref と & の違い

#[derive(Debug)]
pub enum Data {
    Number(i32),
    Text(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ampersand_pattern() {
        // & はパターンで「参照を外す」
        let data = &Data::Number(42);

        match data {
            &Data::Number(n) => {
                // n は i32 型（値がコピーされる）
                println!("& パターン: n = {}, 型 = i32", n);
                assert_eq!(n, 42);
            }
            &Data::Text(ref s) => {
                // Text内のStringは参照として扱う（ムーブを防ぐ）
                println!("& パターン: s = {}", s);
            }
        }
    }

    #[test]
    fn test_ref_pattern() {
        // ref はパターンで「参照を作る」
        let data = Data::Number(42);

        match data {
            Data::Number(ref n) => {
                // n は &i32 型（参照）
                println!("ref パターン: n = {}, 型 = &i32", n);
                assert_eq!(*n, 42);
            }
            Data::Text(ref s) => {
                // s は &String 型（参照）
                println!("ref パターン: s = {}", s);
            }
        }
        // data はまだ使える（ムーブされていない）
        println!("data = {:?}", data);
    }

    #[test]
    fn test_move_pattern() {
        // ref も & も使わない場合
        let data = Data::Text(String::from("こんにちは"));

        match data {
            Data::Number(n) => {
                // n は i32（コピー）
                println!("n = {}", n);
            }
            Data::Text(s) => {
                // s は String（ムーブされる）
                println!("s = {}", s);
            }
        }
        // data はもう使えない（ムーブされた）
        // println!("{:?}", data); // エラー！
    }

    #[test]
    fn test_comparison() {
        let value = 42;
        let reference = &value;

        println!("\n=== 比較 ===");

        // 1. 参照に対して & パターン（参照を外す）
        match reference {
            &n => println!("& パターン: {} (i32型)", n),
        }

        // 2. 値に対して ref パターン（参照を作る）
        match value {
            ref n => println!("ref パターン: {} (&i32型)", n),
        }

        // 3. 通常のパターン
        match value {
            n => println!("通常: {} (i32型)", n),
        }
    }
}
