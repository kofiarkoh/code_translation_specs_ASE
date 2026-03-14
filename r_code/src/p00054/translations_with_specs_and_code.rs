use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                let mut parts = input.split_whitespace();
                let a: i32 = match parts.next() {
                    Some(num) => num.parse().unwrap_or(0), // Default to 0 if parsing fails
                    None => break,
                };
                let b: i32 = match parts.next() {
                    Some(num) => num.parse().unwrap_or(1), // Default to 1 to avoid division by zero
                    None => break,
                };
                let n: i32 = match parts.next() {
                    Some(num) => num.parse().unwrap_or(0), // Default to 0 if parsing fails
                    None => break,
                };

                if b == 0 {
                    // Division by zero is not handled explicitly, as per the spec.
                    break;
                }

                let mut a = a - (a / b) * b; // Adjust a based on b
                let mut sum = 0;

                for _ in 1..=n {
                    a *= 10;
                    let mut j = 9;
                    while j >= 0 {
                        if b * j <= a {
                            break;
                        }
                        j -= 1;
                    }
                    a -= b * j;
                    sum += j;
                }
                println!("{}", sum);
            }
            Err(_) => break, // If reading fails, terminate without output
        }
    }
}
