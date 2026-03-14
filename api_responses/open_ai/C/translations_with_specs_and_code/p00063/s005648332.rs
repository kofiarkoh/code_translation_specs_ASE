use std::io::{self, BufRead};

const N: usize = 100;

fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for j in 0..len / 2 {
        if chars[j] != chars[len - j - 1] {
            return false;
        }
    }
    true
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut cnt = 0;

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Ensure the input does not exceed the maximum length
                if input.len() <= N {
                    if is_palindrome(&input) {
                        cnt += 1;
                    }
                }
            }
            Err(_) => {
                // Handle potential read errors (not specified in the original C code)
                eprintln!("Error reading input");
                break;
            }
        }
    }

    println!("{}", cnt);
}
