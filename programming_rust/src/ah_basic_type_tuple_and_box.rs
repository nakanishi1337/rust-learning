// タプルはスタック領域で、Boxはヒープ領域でデータを管理する例
#[test]
fn stack_vs_heap_with_tuple_and_box() {
    // スタック: タプルはスタックに直接格納される
    let stack_tuple = (1, 2, 3);
    println!("スタック上のタプル: {:p}", &stack_tuple);
    println!("タプルのサイズ: {} bytes", std::mem::size_of_val(&stack_tuple));
    
    // ヒープ: Boxはヒープにデータを格納し、スタックには参照を保持
    let heap_box = Box::new((1, 2, 3));
    println!("ヒープ上のデータへのポインタ: {:p}", &*heap_box);
    println!("Boxのサイズ: {} bytes", std::mem::size_of_val(&heap_box));
    
    // メモリアドレスの違いを確認
    println!("\n--- メモリアドレス比較 ---");
    println!("タプルのアドレス: {:p}", &stack_tuple);
    println!("Boxが指すアドレス: {:p}", &*heap_box);
    
    // タプルの要素にアクセス
    assert_eq!(stack_tuple.0, 1);
    assert_eq!((*heap_box).0, 1);
}

// Boxは動的ディスパッチに使用されることが多い
#[test]
fn box_with_trait_object() {
    trait Animal {
        fn sound(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn sound(&self) { println!("Woof!"); }
    }

    impl Animal for Cat {
        fn sound(&self) { println!("Meow!"); }
    }

    // Vec では「全て同じ型」なので、こうしないといけない
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),  // ✅ 異なる型を一つのコンテナに！
    ];

    for animal in animals {
        animal.sound();
    }
}