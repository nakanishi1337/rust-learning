/// クロージャ章の学習用下書き
///
/// まずはクロージャを使わず、名前付き関数を `sort_by_key` に渡して並べ替える。

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    #[derive(Debug, PartialEq, Eq)]
    struct City {
        name: String,
        population: u64,
    }

    fn city_population_descending(city: &City) -> Reverse<u64> {
        Reverse(city.population)
    }

    #[test]
    fn sort_cities_by_population_without_closure() {
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

        cities.sort_by_key(city_population_descending);

        assert_eq!(cities[0].name, "Tokyo");
        assert_eq!(cities[1].name, "New York");
        assert_eq!(cities[2].name, "London");
    }

    #[test]
    fn sort_cities_by_population_with_closure() {
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

        cities.sort_by_key(|city| Reverse(city.population));

        assert_eq!(cities[0].name, "Tokyo");
        assert_eq!(cities[1].name, "New York");
        assert_eq!(cities[2].name, "London");
    }
}
