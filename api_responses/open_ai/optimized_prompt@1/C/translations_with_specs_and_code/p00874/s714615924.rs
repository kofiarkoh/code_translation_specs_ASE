use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        
        // Read the first two integers
        let w: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if w == 0 {
            break;
        }

        // Initialize counts for both sequences
        let mut a = [0; 21]; // counts for first sequence
        let mut b = [0; 21]; // counts for second sequence

        // Read first sequence
        for _ in 0..w {
            let t: usize = iter.next().unwrap().parse().unwrap();
            a[t] += 1;
        }

        // Read second sequence
        for _ in 0..d {
            let t: usize = iter.next().unwrap().parse().unwrap();
            b[t] += 1;
        }

        // Compute the weighted sum
        let mut res = 0;
        for i in 1..=20 {
            res += i * std::cmp::max(a[i], b[i]);
        }

        // Output the result
        println!("{}", res);
    }
}
