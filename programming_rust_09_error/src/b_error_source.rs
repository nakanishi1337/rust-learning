use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    message: String,
    source: Option<Box<dyn Error>>,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_example() {
        // ソースエラー（原因）を作成
        let cause = MyError {
            message: "root cause".to_string(),
            source: None,
        };

        // メインエラーにソースエラーを含める
        let err: Box<dyn Error> = Box::new(MyError {
            message: "main error".to_string(),
            source: Some(Box::new(cause)),
        });

        // source()でソースエラーを取得
        if let Some(source) = err.source() {
            println!("Cause: {}", source);
        }
    }
}
