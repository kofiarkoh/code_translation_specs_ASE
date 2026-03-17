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

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdout().flush().unwrap(); // Ensure the prompt is printed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input to an integer
    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut i = 0;

    loop {
        // Check for the nearest palindrome less than or equal to n
        if n - i >= 0 {
            if rev(n - i) == n - i {
                println!("{}", n - i);
                break;
            }
        }
        // Check for the nearest palindrome greater than or equal to n
        if rev(n + i) == n + i {
            println!("{}", n + i);
            break;
        }
        i += 1;
    }
}
