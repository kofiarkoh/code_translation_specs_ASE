use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let expression = line.expect("Failed to read line");
        match evaluate_expression(&expression) {
            Some(digit) => println!("{}", digit),
            None => println!("NA"),
        }
    }
}

fn evaluate_expression(expression: &str) -> Option<u8> {
    // Split the expression into parts A, B, and C
    let parts: Vec<&str> = expression.split('=').collect();
    if parts.len() != 2 {
        return None; // Malformed input, but we assume input is well-formed as per spec
    }

    let left_side = parts[0];
    let right_side = parts[1];

    // Split the left side into A and B
    let addends: Vec<&str> = left_side.split('+').collect();
    if addends.len() != 2 {
        return None; // Malformed input, but we assume input is well-formed as per spec
    }

    let a = addends[0];
    let b = addends[1];

    // Check for the presence of 'X' in A or B
    for digit in 0..=9 {
        let digit_char = char::from_digit(digit, 10).unwrap();
        let a_eval = evaluate_with_digit(a, digit_char);
        let b_eval = evaluate_with_digit(b, digit_char);
        let c_eval = right_side.parse::<u32>().ok();

        if let (Some(a_val), Some(b_val), Some(c_val)) = (a_eval, b_eval, c_eval) {
            if a_val + b_val == c_val {
                return Some(digit);
            }
        }
    }

    None
}

fn evaluate_with_digit(part: &str, digit: char) -> Option<u32> {
    let replaced = part.replace('X', &digit.to_string());
    replaced.parse::<u32>().ok()
}
