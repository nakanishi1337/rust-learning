#[test]
fn dont_use_reference() {
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;
    
    fn show(table:Table){
        for (artist, works) in table {
            println!("Artist: {}", artist);
            for work in works {
                println!(" - {}", work);
            }
        }
    }

    let mut table = Table::new();
    table.insert(
        "宮沢賢治".to_string(),
        vec![
            "銀河鉄道の夜".to_string(),
            "注文の多い料理店".to_string(),
        ],
    );
    table.insert(
        "芥川龍之介".to_string(),
        vec![
            "羅生門".to_string(),
            "地獄変".to_string(),
        ],
    );
    show(table);
    // show(table); // showにtableを渡したので、ここでコンパイルエラーになる
}

#[test]
fn use_reference() {
    use std::collections::HashMap;
    type Table = HashMap<String, Vec<String>>;

    fn show(table:&Table){
        for (artist, works) in table {
            println!("Artist: {}", artist);
            for work in works {
                println!(" - {}", work);
            }
        }
    }

    let mut table = Table::new();
    table.insert(
        "宮沢賢治".to_string(),
        vec![
            "銀河鉄道の夜".to_string(),
            "注文の多い料理店".to_string(),
        ],
    );
    table.insert(
        "芥川龍之介".to_string(),
        vec![
            "羅生門".to_string(),
            "地獄変".to_string(),
        ],
    );
    show(&table);
    show(&table); // 参照を渡しているので、ここでも問題なく動作する

}