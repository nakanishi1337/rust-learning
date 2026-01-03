// 3種類のヴァリアントを持つ列挙型の例
#[derive(Debug, PartialEq)]
pub enum Message {
    // 1. データなしヴァリアント（Unit variant）
    Quit,

    // 2. タプル型ヴァリアント（Tuple variant）
    Move(i32, i32),

    // 3. 構造体型ヴァリアント（Struct variant）
    Write { text: String, color: String },
}

impl Message {
    // メッセージの内容を表示する関数
    pub fn display(&self) {
        match self {
            Message::Quit => println!("終了します"),
            Message::Move(x, y) => println!("座標 ({}, {}) に移動", x, y),
            Message::Write { text, color } => {
                println!("「{}」を{}色で表示", text, color)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_variant() {
        // データなしヴァリアント
        let msg = Message::Quit;
        msg.display();
        assert_eq!(msg, Message::Quit);
    }

    #[test]
    fn test_tuple_variant() {
        // タプル型ヴァリアント
        let msg = Message::Move(10, 20);
        msg.display();

        // パターンマッチでデータを取得
        if let Message::Move(x, y) = msg {
            assert_eq!(x, 10);
            assert_eq!(y, 20);
        }
    }

    #[test]
    fn test_struct_variant() {
        // 構造体型ヴァリアント
        let msg = Message::Write {
            text: String::from("こんにちは"),
            color: String::from("赤"),
        };
        msg.display();

        // パターンマッチでフィールドにアクセス
        if let Message::Write { text, color } = msg {
            assert_eq!(text, "こんにちは");
            assert_eq!(color, "赤");
        }
    }

    #[test]
    fn test_match_all_variants() {
        let messages = vec![
            Message::Quit,
            Message::Move(100, 200),
            Message::Write {
                text: String::from("テスト"),
                color: String::from("青"),
            },
        ];

        for msg in messages {
            match msg {
                Message::Quit => println!("✓ Quitヴァリアント"),
                Message::Move(x, y) => {
                    println!("✓ Moveヴァリアント: ({}, {})", x, y);
                    assert!(x >= 0 && y >= 0);
                }
                Message::Write { text, color } => {
                    println!("✓ Writeヴァリアント: {} ({})", text, color);
                    assert!(!text.is_empty());
                }
            }
        }
    }
}
