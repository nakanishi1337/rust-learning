// 演算子オーバーロードの学習テスト
// 実装とテストを近く配置して学習しやすくする

use std::ops::{Add, Index, IndexMut, Neg, SubAssign};

// ========== 1. 単項演算子（Neg）の例 ==========
/// 複素数型：単項演算子 `-` を実装
#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }
}

// 単項マイナス演算子を実装: -z (複素数の符号反転)
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

#[cfg(test)]
mod tests_neg {
    use super::*;

    #[test]
    fn test_unary_negation_complex() {
        let z = Complex::new(3.0, 4.0);
        let negated = -z;

        assert_eq!(negated, Complex::new(-3.0, -4.0));
        println!("✓ 単項演算子（Neg）: -{:?} = {:?}", z, negated);
    }

    #[test]
    fn test_double_negation() {
        let z = Complex::new(5.0, -2.0);
        let double_neg = -(-z);

        assert_eq!(double_neg, z);
        println!("✓ 二重否定: -(-{:?}) = {:?}", z, double_neg);
    }
}

// ========== 2. 二項演算子（Add）の例 ==========
/// 2次元ベクトル型：二項演算子 `+` を実装
#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Vector2D {
    fn new(x: f64, y: f64) -> Self {
        Vector2D { x, y }
    }
}

// 二項加算演算子を実装: v1 + v2
impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[cfg(test)]
mod tests_add {
    use super::*;

    #[test]
    fn test_binary_addition_vector() {
        let v1 = Vector2D::new(3.0, 4.0);
        let v2 = Vector2D::new(1.0, 2.0);
        let result = v1 + v2;

        assert_eq!(result, Vector2D::new(4.0, 6.0));
        println!("✓ 二項演算子（Add）: {:?} + {:?} = {:?}", v1, v2, result);
    }

    #[test]
    fn test_multiple_additions() {
        let v1 = Vector2D::new(1.0, 2.0);
        let v2 = Vector2D::new(3.0, 4.0);
        let v3 = Vector2D::new(5.0, 6.0);
        let result = v1 + v2 + v3;

        assert_eq!(result, Vector2D::new(9.0, 12.0));
        println!("✓ 複数加算: {:?} + {:?} + {:?} = {:?}", v1, v2, v3, result);
    }
}

// ========== 3. 複合代入演算子（SubAssign）の例 ==========
/// マトリックス行タイプ：複合代入演算子 `-=` を実装
#[derive(Debug, Clone, PartialEq)]
struct Row {
    values: Vec<f64>,
}

impl Row {
    fn new(values: Vec<f64>) -> Self {
        Row { values }
    }
}

// 複合代入演算子を実装: row1 -= row2 (要素ごとの減算)
impl SubAssign for Row {
    fn sub_assign(&mut self, other: Row) {
        if self.values.len() != other.values.len() {
            panic!("異なるサイズの行は減算できません");
        }
        for (a, b) in self.values.iter_mut().zip(other.values.iter()) {
            *a -= b;
        }
    }
}

#[cfg(test)]
mod tests_subassign {
    use super::*;

    #[test]
    fn test_compound_assignment_row() {
        let mut row1 = Row::new(vec![10.0, 20.0, 30.0]);
        let row2 = Row::new(vec![1.0, 2.0, 3.0]);

        row1 -= row2;

        assert_eq!(row1, Row::new(vec![9.0, 18.0, 27.0]));
        println!("✓ 複合代入演算子（SubAssign）: row1 -= row2 → {:?}", row1);
    }

    #[test]
    fn test_multiple_subassign() {
        let mut row = Row::new(vec![100.0, 50.0, 75.0]);
        let sub1 = Row::new(vec![10.0, 5.0, 25.0]);
        let sub2 = Row::new(vec![20.0, 15.0, 10.0]);

        row -= sub1;
        row -= sub2;

        assert_eq!(row, Row::new(vec![70.0, 30.0, 40.0]));
        println!("✓ 複数回の -= 適用: 結果 = {:?}", row);
    }
}

// ========== 4. インデックス操作（Index/IndexMut）の例 ==========
/// 動的配列タイプ：Index と IndexMut を実装
#[derive(Debug, Clone)]
struct DynamicArray {
    data: Vec<i32>,
}

impl DynamicArray {
    fn new(data: Vec<i32>) -> Self {
        DynamicArray { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

// 読み取り用インデックスアクセスを実装: arr[i]
impl Index<usize> for DynamicArray {
    type Output = i32;

    fn index(&self, index: usize) -> &i32 {
        if index >= self.data.len() {
            panic!(
                "インデックス {} は範囲外です（長さ: {}）",
                index,
                self.data.len()
            );
        }
        &self.data[index]
    }
}

// 書き込み用インデックスアクセスを実装: arr[i] = value
impl IndexMut<usize> for DynamicArray {
    fn index_mut(&mut self, index: usize) -> &mut i32 {
        if index >= self.data.len() {
            panic!(
                "インデックス {} は範囲外です（長さ: {}）",
                index,
                self.data.len()
            );
        }
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests_index {
    use super::*;

    #[test]
    fn test_index_read_access() {
        let arr = DynamicArray::new(vec![10, 20, 30, 40, 50]);

        assert_eq!(arr[0], 10);
        assert_eq!(arr[2], 30);
        assert_eq!(arr[4], 50);
        println!(
            "✓ インデックス読み取り: arr[0]={}, arr[2]={}, arr[4]={}",
            arr[0], arr[2], arr[4]
        );
    }

    #[test]
    fn test_index_write_access() {
        let mut arr = DynamicArray::new(vec![1, 2, 3, 4, 5]);

        arr[0] = 100;
        arr[2] = 300;
        arr[4] = 500;

        assert_eq!(arr[0], 100);
        assert_eq!(arr[2], 300);
        assert_eq!(arr[4], 500);
        println!("✓ インデックス書き込み: arr = {:?}", arr.data);
    }

    #[test]
    fn test_index_modify_in_loop() {
        let mut arr = DynamicArray::new(vec![1, 2, 3, 4, 5]);

        // すべての要素を2倍にする
        for i in 0..arr.len() {
            arr[i] *= 2;
        }

        assert_eq!(arr.data, vec![2, 4, 6, 8, 10]);
        println!("✓ ループでのインデックス変更: arr = {:?}", arr.data);
    }

    #[test]
    #[should_panic(expected = "範囲外")]
    fn test_index_out_of_bounds() {
        let arr = DynamicArray::new(vec![1, 2, 3]);
        let _ = arr[5]; // パニック発生
    }
}
