use std::io::{self, Write};

fn rev(n: i32) -> i32 {
    let mut out = 0;
    let mut num = n;

    while num > 0 {
        out = out * 10 + (num % 10);
        num /= 10;
    }

    out
}

fn nearest_palindrome(n: i32) -> i32 {
    let mut i = 0;

    loop {
        if n - i >= 0 {
            let candidate = n - i;
            if rev(candidate) == candidate {
                return candidate;
            }
        }
        let candidate = n + i;
        if rev(candidate) == candidate {
            return candidate;
        }
        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: i32 = input.trim().parse().expect("Input must be a non-negative integer");

    if n < 0 {
        eprintln!("Input must be a non-negative integer");
        return;
    }

    let palindrome = nearest_palindrome(n);
    println!("{}", palindrome);
}
