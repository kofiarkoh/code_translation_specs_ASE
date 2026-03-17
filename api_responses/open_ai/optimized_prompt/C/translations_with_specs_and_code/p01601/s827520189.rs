use std::io::{self, Write};

fn rev(n: u32) -> u32 {
    let mut out = 0;
    let mut num = n;

    while num > 0 {
        out = out * 10 + (num % 10);
        num /= 10;
    }

    out
}

fn nearest_palindrome(n: u32) -> u32 {
    let mut i = 0;

    loop {
        if n >= i {
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
    
    let n: u32 = input.trim().parse().expect("Input must be a non-negative integer");

    let result = nearest_palindrome(n);
    println!("{}", result);
}
