/// Default トレイトの学習
///
/// Defaultトレイトを実装すると、型のデフォルト値を定義できる

#[derive(Debug, PartialEq)]
struct Config {
    host: String,
    port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            host: String::from("localhost"),
            port: 8080,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = Config::default();

        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);

        println!("✅ Default: {:?}", config);
    }
}
