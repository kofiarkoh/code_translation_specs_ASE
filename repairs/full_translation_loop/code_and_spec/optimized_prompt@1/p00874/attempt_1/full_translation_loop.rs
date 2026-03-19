use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut iter = line.split_whitespace();

        // Read the first two integers
        let w: usize = match iter.next().unwrap().parse() {
            Ok(num) if num > 0 => num,
            _ => break, // Termination condition
        };
        let d: usize = iter.next().unwrap().parse().expect("Invalid input");

        // Initialize counts for the two sequences
        let mut a = [0; 21]; // Counts for the first sequence
        let mut b = [0; 21]; // Counts for the second sequence

        // Read the first sequence
        for _ in 0..w {
            let t: usize = iter.next().unwrap().parse().expect("Invalid input");
            a[t] += 1;
        }

        // Read the second sequence
        for _ in 0..d {
            let t: usize = iter.next().unwrap().parse().expect("Invalid input");
            b[t] += 1;
        }

        // Compute the weighted sum
        let mut res = 0;
        for i in 1..=20 {
            res += i * a[i].max(b[i]);
        }

        // Output the result
        println!("{}", res);
    }
}
