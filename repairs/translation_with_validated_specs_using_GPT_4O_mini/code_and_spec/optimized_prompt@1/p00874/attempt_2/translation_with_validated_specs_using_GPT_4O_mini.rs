use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let w: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        // Termination condition
        if w == 0 {
            break;
        }

        let mut a = [0; 21]; // Counts for the first set
        let mut b = [0; 21]; // Counts for the second set

        // Read first set
        for _ in 0..w {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            a[t] += 1;
        }

        // Read second set
        for _ in 0..d {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            b[t] += 1;
        }

        // Calculate the weighted sum
        let mut res = 0;
        for i in 1..=20 {
            res += i * a[i].max(b[i]);
        }

        // Output the result
        println!("{}", res);
    }
}
