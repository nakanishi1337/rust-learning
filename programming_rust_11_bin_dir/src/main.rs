// src/bin/ディレクトリの使い方
//
// src/bin/配下に配置した各Rustファイルは、独立した実行ファイルとして扱われます。
// 実行方法: cargo run --bin <ファイル名>
//
// 例:
//   cargo run --bin hello         # src/bin/hello.rs を実行
//   cargo run --bin add           # src/bin/add.rs を実行
//   cargo run --bin loop_example  # src/bin/loop_example.rs を実行
//
// このmain.rsは通常通り `cargo run` で実行されます。

fn main() {
    println!("Hello, world!");
    println!("bin/配下のファイルを実行するには: cargo run --bin <ファイル名>");
}
