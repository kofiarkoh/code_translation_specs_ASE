use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        // Read each line and parse it as an integer
        if let Ok(input) = line {
            if let Ok(s) = input.trim().parse::<i32>() {
                // Ensure the input is non-negative as per the specification
                if s >= 0 {
                    let mut ans = 0;

                    // Iterate through possible combinations of non-negative integers
                    for a in 0..=1000 {
                        if s < a {
                            break; // No need to continue if s < a
                        }
                        for b in 0..=1000 {
                            if s < a + b {
                                break; // No need to continue if s < a + b
                            }
                            ans += 501 - (500 - (s - a - b)).abs();
                        }
                    }
                    // Output the computed cumulative metric
                    println!("{}", ans);
                }
            }
        }
    }
}
