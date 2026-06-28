/// クロージャと呼び出しトレイトの違い
///
/// クロージャは捕捉した値の使い方によって、`Fn`, `FnMut`, `FnOnce` のどれかとして扱われる。

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    fn call_fn<F>(f: F) -> i32
    where
        F: Fn() -> i32,
    {
        f() + f()
    }

    fn call_fn_mut<F>(mut f: F) -> i32
    where
        F: FnMut() -> i32,
    {
        f() + f()
    }

    fn call_fn_once<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        f()
    }

    fn write_user_index(users: HashMap<u64, String>) -> Vec<String> {
        users.into_values().collect()
    }

    #[test]
    fn fn_reads_captured_value_without_changing_it() {
        let number = 10;

        let read_number = || number;

        assert_eq!(call_fn(read_number), 20);
        assert_eq!(number, 10);
    }

    #[test]
    fn fn_mut_changes_captured_value() {
        let mut number = 0;

        let mut increment = || {
            number += 1;
            number
        };

        assert_eq!(call_fn_mut(&mut increment), 3);
        assert_eq!(number, 2);
    }

    #[test]
    fn fn_once_consumes_captured_hash_map() {
        let users = HashMap::from([
            (1, "Alice".to_string()),
            (2, "Bob".to_string()),
            (3, "Carol".to_string()),
        ]);

        let export_users = || write_user_index(users);

        let mut names = call_fn_once(export_users);
        names.sort();

        assert_eq!(names, vec!["Alice", "Bob", "Carol"]);
    }
}
