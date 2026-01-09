/// Sizedトレイトの学習
///
/// Sizedトレイトは、コンパイル時にサイズが決まっている型を表す
/// ジェネリック関数は暗黙的に T: Sized という境界を持つ
/// ?Sized を使うと、サイズが不定な型（str, [T], dyn Trait）も受け入れられる
use std::mem::size_of;

// デフォルト: T は Sized を持つ
fn print_sized<T>(value: &T) {
    println!("Sized型: {} bytes", size_of::<T>());
}

// ?Sized を使うと unsized 型も受け入れられる
fn print_maybe_unsized<T: ?Sized>(value: &T) {
    println!("?Sized型: {} bytes", std::mem::size_of_val(value));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sized_vs_unsized() {
        println!("\n--- Sized vs ?Sized ---");

        // Sized な型
        let num = 42i32;
        print_sized(&num); // OK
        print_maybe_unsized(&num); // OK

        // Unsized な型（str）
        let text: &str = "Hello";
        // print_sized(text); // エラー！str は Sized ではない
        print_maybe_unsized(text); // OK: ?Sized だから受け入れられる

        println!("\n&str 参照自体は Sized: {} bytes", size_of::<&str>());
        println!(
            "str データは Unsized: {} bytes",
            std::mem::size_of_val(text)
        );

        assert_eq!(size_of::<i32>(), 4);
        assert_eq!(size_of::<&str>(), 16); // ファットポインタ
    }
}
