use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for number of pairs");

    // Preconditions: n must be positive
    if n == 0 {
        return; // No pairs to process
    }

    let mut c1 = vec![0; n];
    let mut c2 = vec![0; n];

    // Read pairs of integers
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid integer input"))
            .collect();
        c1[i] = nums[0];
        c2[i] = nums[1];
    }

    let mut s = 0;
    let mut amax = 0;
    let mut amaxi = 0;

    for i in 0..n - 1 {
        for j in 0..(n - i - 1) {
            let a = c2[j] * c1[j + 1];
            if a > amax || (a == amax && c1[j] * c2[j + 1] < c1[amaxi] * c2[amaxi + 1]) {
                amax = a;
                amaxi = j;
            }
        }
        s += c1[amaxi] * c2[amaxi] * c1[amaxi + 1] * c2[amaxi + 1];
        c2[amaxi] = c2[amaxi + 1];
        for j in (amaxi + 1)..(n - i) {
            c1[j] = c1[j + 1];
            c2[j] = c2[j + 1];
        }
        amax = 0;
    }

    // Output the result
    println!("{}", s);
}
