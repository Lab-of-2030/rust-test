// filepath: /opt/git/rust-test/src/question1.rs
use std::env;

/// Determines the output for a given number based on FizzBuzz rules.
fn fizzbuzz_output(n: i32) -> String {
    let mut output = n.to_string();

    if n % 3 == 0 && n % 5 == 0 {
        output.push_str(" FizzBuzz");
    } else if n % 3 == 0 {
        output.push_str(" Fizz");
    } else if n % 5 == 0 {
        output.push_str(" Buzz");
    }

    output
}

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let n: i32 = if args.len() > 1 {
        args[1].parse().unwrap_or(5)
    } else {
        5
    };

    for i in 1..=n {
        println!("{}", fizzbuzz_output(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz_output() {
        assert_eq!(fizzbuzz_output(1), "1");
        assert_eq!(fizzbuzz_output(3), "3 Fizz");
        assert_eq!(fizzbuzz_output(5), "5 Buzz");
        assert_eq!(fizzbuzz_output(15), "15 FizzBuzz");
        assert_eq!(fizzbuzz_output(2), "2");
        assert_eq!(fizzbuzz_output(6), "6 Fizz");
        assert_eq!(fizzbuzz_output(10), "10 Buzz");
        assert_eq!(fizzbuzz_output(30), "30 FizzBuzz");
    }

    #[test]
    fn test_fizzbuzz_output_edge_cases() {
        assert_eq!(fizzbuzz_output(0), "0 FizzBuzz"); // Divisible by both 3 and 5
        assert_eq!(fizzbuzz_output(-3), "-3 Fizz"); // Negative number divisible by 3
        assert_eq!(fizzbuzz_output(-5), "-5 Buzz"); // Negative number divisible by 5
        assert_eq!(fizzbuzz_output(-15), "-15 FizzBuzz"); // Negative number divisible by both
    }
}
