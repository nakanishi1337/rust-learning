/// `move` クロージャの学習用下書き
///
/// `thread::spawn` に渡すクロージャは別スレッドで実行されるため、
/// ローカル変数を借用したままにはできない。
/// `move` を付けて、ソート対象の `cities` の所有権をスレッドへ渡す。
/// この例では `cities` 自体は返さず、ソート後の都市名だけを返す。

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;
    use std::thread;

    #[derive(Debug, PartialEq, Eq)]
    struct City {
        name: String,
        population: u64,
    }

    #[test]
    fn sort_cities_in_thread_with_move_closure() {
        let mut cities = vec![
            City {
                name: "Tokyo".to_string(),
                population: 37_194_000,
            },
            City {
                name: "London".to_string(),
                population: 9_748_000,
            },
            City {
                name: "New York".to_string(),
                population: 18_937_000,
            },
        ];

        let handle = thread::spawn(move || {
            cities.sort_by_key(|city| Reverse(city.population));

            cities
                .iter()
                .map(|city| city.name.clone())
                .collect::<Vec<_>>()
        });

        let sorted_city_names = handle.join().unwrap();

        assert_eq!(sorted_city_names, vec!["Tokyo", "New York", "London"]);
    }
}
