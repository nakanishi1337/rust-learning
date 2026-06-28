fn main() {
    let lucky_number = 7; // I’m feeling lucky today(今日はラッキーな気がするよ)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_runs() {
        main();
    }
}
