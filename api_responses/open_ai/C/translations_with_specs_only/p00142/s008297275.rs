use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let n: usize = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Terminate on zero input
        }

        let mut distinct_differences = HashSet::new();

        // Calculate distinct differences for values from 1 to (n-1)/2
        for i in 1..=(n - 1) / 2 {
            let squared_i = (i * i) % n; // Square and take modulo n
            for j in 1..=i {
                let squared_j = (j * j) % n; // Square and take modulo n
                let difference = (squared_i as isize - squared_j as isize).abs() % n; // Calculate absolute difference modulo n
                distinct_differences.insert(difference);
            }
        }

        // Output the count of distinct differences
        println!("{}", distinct_differences.len());
    }
}
