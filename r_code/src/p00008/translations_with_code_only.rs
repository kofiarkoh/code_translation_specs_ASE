use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: i32 = match line {
            Ok(l) => match l.trim().parse() {
                Ok(num) => num,
                Err(_) => continue, // Skip invalid input
            },
            Err(_) => break, // Break on read error
        };

        let mut count = 0;

        // Iterate over all combinations of a, b, c, d from 0 to 9
        for a in 0..=9 {
            for b in 0..=9 {
                for c in 0..=9 {
                    for d in 0..=9 {
                        if a + b + c + d == n {
                            count += 1;
                        }
                    }
                }
            }
        }

        // Print the result for the current n
        println!("{}", count);
    }
}
