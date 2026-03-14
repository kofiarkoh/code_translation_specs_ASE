use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 2 {
            continue; // Skip invalid input
        }

        let numerator = nums[0];
        let denominator = nums[1];

        // Preconditions check
        if numerator < 0 || denominator <= 0 {
            continue; // Skip invalid input
        }

        let (integer_part, decimal_part, repeat_index) = divide_and_find_repeats(numerator, denominator);
        let decimal_representation = format!("{}.{}", integer_part, decimal_part);
        println!("{}", decimal_representation);

        if let Some(index) = repeat_index {
            let caret_line = " ".repeat(index + integer_part.to_string().len() + 1) + "^";
            println!("{}", caret_line);
        }
    }
}

fn divide_and_find_repeats(numerator: i32, denominator: i32) -> (i32, String, Option<usize>) {
    let integer_part = numerator / denominator;
    let mut remainder = numerator % denominator;

    if remainder == 0 {
        return (integer_part, String::new(), None); // No decimal part
    }

    let mut decimal_part = String::new();
    let mut remainders = std::collections::HashMap::new();
    let mut repeat_index = None;

    while remainder != 0 {
        if let Some(&index) = remainders.get(&remainder) {
            repeat_index = Some(index);
            break;
        }

        remainders.insert(remainder, decimal_part.len());
        remainder *= 10;
        decimal_part.push(char::from_digit((remainder / denominator) as u32, 10).unwrap());
        remainder %= denominator;
    }

    (integer_part, decimal_part, repeat_index)
}
