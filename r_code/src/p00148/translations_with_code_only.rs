use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line and attempt to parse it as an integer
        if let Ok(line) = line {
            if let Ok(n) = line.trim().parse::<i32>() {
                let mut n = n % 39;
                if n == 0 {
                    n = 39;
                }
                // Print formatted output
                println!("3C{:02}", n);
            }
        } else {
            // If reading the line fails, we can break out of the loop
            break;
        }
    }
}
