// smallestの返り値と引数は同じライフタイムを持つ必要がある
// smallest<><'a>(v: &'a Vec<i32>) -> &'a i32 のように解釈される
fn smallest(v: &Vec<i32>) -> &i32 {
    let mut smallest = &v[0];
    for i in v {
        if i < smallest {
            smallest = i;
        }
    }
    smallest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_smallest() {
        let s;
        {
            let numbers = vec![34, 50, 25, 100, 65];
            s = smallest(&numbers);
            assert_eq!(*s, 25);
        }
        // assert_eq!(*s, 25); // エラー: numbersのライフタイムが終わっているため参照できない
    }
}
