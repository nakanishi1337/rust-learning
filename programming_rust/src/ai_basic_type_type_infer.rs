// 型推論

fn build_vector() -> Vec<i16> {
    let mut v : Vec<i16> = Vec::new();
    v.push(10i16);
    v.push(20i16);
    v
}

fn build_vector_infer() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

#[test]
fn test_build_vector() {
    assert_eq!(build_vector(), build_vector_infer());
}