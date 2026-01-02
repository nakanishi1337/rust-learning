// 内部可変性（Interior Mutability）
// イミュータブルな参照経由でも内部の値を変更できる仕組み

use std::cell::{Cell, RefCell};

// Cell<T>: Copy 型に使える、簡単な内部可変性
struct Counter {
    count: Cell<i32>,
}

impl Counter {
    fn new() -> Self {
        Counter {
            count: Cell::new(0),
        }
    }

    // &self でも内部の値を変更できる！
    fn increment(&self) {
        let current = self.count.get();
        self.count.set(current + 1);
    }

    fn get(&self) -> i32 {
        self.count.get()
    }
}

// RefCell<T>: 参照を借りる形の内部可変性
struct Logger {
    logs: RefCell<Vec<String>>,
}

impl Logger {
    fn new() -> Self {
        Logger {
            logs: RefCell::new(Vec::new()),
        }
    }

    // &self でも内部のベクタに追加できる！
    fn log(&self, message: &str) {
        self.logs.borrow_mut().push(message.to_string());
    }

    fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cell_example() {
        let counter = Counter::new();

        println!("初期値: {}", counter.get());

        // &counter（イミュータブル参照）なのに値を変更できる
        counter.increment();
        counter.increment();
        counter.increment();

        println!("3回インクリメント後: {}", counter.get());
        assert_eq!(counter.get(), 3);
    }

    #[test]
    fn refcell_example() {
        let logger = Logger::new();

        // &logger（イミュータブル参照）なのに内部のVecに追加できる
        logger.log("初期化しました");
        logger.log("処理を開始");
        logger.log("処理完了");

        let logs = logger.get_logs();
        println!("ログ:");
        for log in &logs {
            println!("  - {}", log);
        }

        assert_eq!(logs.len(), 3);
        assert_eq!(logs[0], "初期化しました");
    }
}
