// 定数パラメータを持つ構造体（Const Generics）
// const N: usize のように定数を型パラメータとして受け取る

// N は定数パラメータ：配列のサイズ
struct FixedArray<T, const N: usize> {
    data: [T; N],
}

impl<T, const N: usize> FixedArray<T, N> {
    fn new(data: [T; N]) -> Self {
        FixedArray { data }
    }

    fn len(&self) -> usize {
        N // 定数パラメータを直接使える
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_sizes() {
        // 異なるサイズの配列を作成
        let arr3 = FixedArray::new([1, 2, 3]);
        let arr5 = FixedArray::new([10, 20, 30, 40, 50]);

        println!("arr3 のサイズ: {}", arr3.len());
        println!("arr5 のサイズ: {}", arr5.len());

        assert_eq!(arr3.len(), 3);
        assert_eq!(arr5.len(), 5);

        // arr3 と arr5 は異なる型！
        // FixedArray<i32, 3> と FixedArray<i32, 5>
    }

    #[test]
    fn access_elements() {
        let arr = FixedArray::new([100, 200, 300]);

        println!("arr[0] = {:?}", arr.get(0));
        println!("arr[1] = {:?}", arr.get(1));
        println!("arr[2] = {:?}", arr.get(2));

        assert_eq!(arr.get(0), Some(&100));
        assert_eq!(arr.get(1), Some(&200));
        assert_eq!(arr.get(2), Some(&300));
        assert_eq!(arr.get(3), None);
    }

    #[test]
    fn string_array() {
        let arr = FixedArray::new(["Alice", "Bob"]);

        println!("文字列配列のサイズ: {}", arr.len());
        println!("arr[0] = {:?}", arr.get(0));

        assert_eq!(arr.len(), 2);
        assert_eq!(arr.get(0), Some(&"Alice"));
    }
}
