//// 配列のテスト///////////////////////////////////

#[test]
fn test_array_index() {
    let lazy_caterer = [1, 2, 4, 7, 11, 16];
    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];

    assert_eq!(lazy_caterer[3], 7);
    assert_eq!(taxonomy.len(), 3);
}

#[test]
fn test_long_array() {
    // エラトステネスの篩
    let mut sieve = [true; 10000];
    for i in 2..100{
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }
    assert!(sieve[211]);
    assert!(!sieve[9876]);
}

#[test]
fn test_array_methods() {
    let mut numbers = [50, 20, 10, 40, 30];
    numbers.sort(); // 実際にはスライスのメソッドを呼び出している
    assert_eq!(numbers, [10, 20, 30, 40, 50]);   
}


//// ベクタのテスト///////////////////////////////////
/// 配列はスタックで固定長だが、ベクタはヒープに確保される可変長配列

#[test]
fn test_vector_push() {
    let mut primes = vec![2, 3, 5, 7];
    primes.push(11);
    primes.push(13);
    assert_eq!(primes.len(), 6);
    assert_eq!(primes[4], 11);
}

#[test]
fn test_vector_init() {
    let zeros = vec![0; 1000];
    assert_eq!(zeros.len(), 1000);
}

#[test]
fn test_vector_collect() {
    let evens: Vec<i32> = (0..10).filter(|x| x % 2 == 0).collect();
    assert_eq!(evens, vec![0, 2, 4, 6, 8]);
}

//ベクタはヒープに確保したサイズを超えると自動的により大きな領域を確保し直す
#[test]
fn test_vector_capacity() {
    let mut vec = Vec::new();
    let mut capacity = vec.capacity();
    for i in 0..100 {
        vec.push(i);
        if vec.capacity() != capacity {
            capacity = vec.capacity();
            println!("Capacity increased to {}", capacity);
        }
    }
    assert_eq!(vec.len(), 100);
}

//あらかじめwith_capacityで確保しておくこともできる
#[test]
fn test_vector_with_capacity() {
    let mut vec = Vec::with_capacity(100);
    assert_eq!(vec.capacity(), 100);
    for i in 0..100 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 100);
    assert_eq!(vec.capacity(), 100);
}


//// スライスのテスト///////////////////////////////////

#[test]
fn test_slice_pointer() {
    let v=vec![10, 20, 30, 40, 50];
    let a = [10, 20, 30, 40, 50];

    let sv: &[i32] = &v; // ベクタからスライスを作成
    let sa: &[i32] = &a; // 配列からスライスを作成

    assert_eq!(sv, &[10, 20, 30, 40, 50]);
    assert_eq!(sa, &[10, 20, 30, 40, 50]);
}

#[test]
fn test_work_both_vector_and_array() {

    // スライスを引数に取る関数にしておけば、ベクタと配列の両方で動作する
    fn test_work_both_vector_and_array(slice: &[i32]) -> i32 {
        slice.iter().sum()
    }

    let v = vec![10, 20, 30, 40, 50];
    let a = [10, 20, 30, 40, 50];

    assert_eq!(test_work_both_vector_and_array(&v), 150);
    assert_eq!(test_work_both_vector_and_array(&a), 150);
}
