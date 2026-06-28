use std::cmp::Ordering;

fn compare_guess(guess: u32, secret_number: u32) -> Ordering {
    guess.cmp(&secret_number)
}

fn parse_guess(input: &str) -> Option<u32> {
    input.trim().parse().ok()
}

fn main() {
    let secret_number = 42;
    let guess = parse_guess("42").expect("valid number");

    match compare_guess(guess, secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_guess() {
        assert_eq!(parse_guess(" 42\n"), Some(42));
    }

    #[test]
    fn reject_invalid_guess() {
        assert_eq!(parse_guess("not a number"), None);
    }

    #[test]
    fn compare_guess_to_secret_number() {
        assert_eq!(compare_guess(10, 42), Ordering::Less);
        assert_eq!(compare_guess(99, 42), Ordering::Greater);
        assert_eq!(compare_guess(42, 42), Ordering::Equal);
    }

    #[test]
    fn example_runs_without_waiting_for_input() {
        main();
    }
}
